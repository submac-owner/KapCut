#!/usr/bin/env python3
"""Decode editing patterns from a reference video into a style profile.

Uses only local CPU ffmpeg/ffprobe. Detects:
  - shot boundaries + cut cadence (scene detection)
  - average color per shot (downscaled frame sampling)
  - loudness / music presence (volumedetect)
  - frame rate, resolution, aspect, duration

The profile can be fed to style_engine.invent() so the agent can reproduce
the reference's rhythm and look on the current project.
"""
import json, re, subprocess, sys, pathlib, statistics

FFMPEG = "ffmpeg"
FFPROBE = "ffprobe"


class Result:
    def __init__(self, p):
        self.stdout = p.stdout           # bytes (needed for rawvideo)
        self.stderr = p.stderr.decode("utf-8", "replace")


def _run(args):
    return Result(subprocess.run(args, capture_output=True))


def probe(path):
    r = _run([FFPROBE, "-v", "error", "-show_streams", "-show_format", "-of", "json", path])
    d = json.loads((r.stdout or b"{}").decode("utf-8", "replace"))
    v = next((s for s in d.get("streams", []) if s.get("codec_type") == "video"), {})
    a = next((s for s in d.get("streams", []) if s.get("codec_type") == "audio"), None)
    rate = v.get("avg_frame_rate", "0/1").split("/")
    fps = float(rate[0]) / float(rate[1]) if len(rate) == 2 and float(rate[1]) else 0.0
    return {
        "width": v.get("width"), "height": v.get("height"), "fps": round(fps, 3),
        "duration": round(float(d.get("format", {}).get("duration", 0)), 3),
        "aspect": round(v.get("width", 1) / max(1, v.get("height", 1)), 3),
        "has_audio": a is not None,
    }


def scenes(path, threshold=0.25):
    r = _run([FFMPEG, "-hide_banner", "-i", path,
              "-vf", f"select='gt(scene,{threshold})',showinfo", "-an", "-f", "null", "-"])
    times = [float(m) for m in re.findall(r"pts_time:([0-9.]+)", r.stderr or "")]
    return sorted(set(round(t, 3) for t in times))


def average_color(path, at):
    r = _run([FFMPEG, "-v", "error", "-ss", str(at), "-i", path, "-frames:v", "1",
              "-vf", "scale=1:1", "-pix_fmt", "rgb24", "-f", "rawvideo", "-"])
    raw = r.stdout or b""
    return [int(raw[0]), int(raw[1]), int(raw[2])] if len(raw) >= 3 else [0, 0, 0]


def loudness(path):
    r = _run([FFMPEG, "-hide_banner", "-i", path, "-af", "volumedetect", "-f", "null", "-"])
    m = re.search(r"mean_volume:\s*(-?[0-9.]+) dB", r.stderr or "")
    return float(m.group(1)) if m else None


def decode(path, threshold=0.25):
    meta = probe(path)
    cuts = scenes(path, threshold)
    bounds = [0.0] + cuts + [meta["duration"]]
    shots = []
    for i in range(len(bounds) - 1):
        start, end = bounds[i], bounds[i + 1]
        if end - start < 0.15:
            continue
        mid = (start + end) / 2
        shots.append({"start": round(start, 3), "duration": round(end - start, 3),
                      "color": average_color(path, mid)})
    lengths = [s["duration"] for s in shots]
    colors = [s["color"] for s in shots] or [[0, 0, 0]]
    mean_rgb = [round(sum(c[i] for c in colors) / len(colors)) for i in range(3)]
    return {
        "source": str(path), "meta": meta,
        "style_profile": {
            "shots": len(shots),
            "cut_times": cuts,
            "shot_lengths": [round(x, 3) for x in lengths],
            "mean_shot": round(statistics.mean(lengths), 3) if lengths else meta["duration"],
            "median_shot": round(statistics.median(lengths), 3) if lengths else meta["duration"],
            "pace": "fast" if lengths and statistics.median(lengths) < 2 else
                    ("medium" if lengths and statistics.median(lengths) < 6 else "slow"),
            "mean_rgb": mean_rgb,
            "loudness_db": loudness(path),
            "bpm_estimate": None,
        },
        "shots": shots,
    }


if __name__ == "__main__":
    p = sys.argv[1] if len(sys.argv) > 1 else r"C:\Users\Admin\Desktop\Capcut\targets\sample-20s-720p.mp4"
    print(json.dumps(decode(p), indent=2)[:3000])
