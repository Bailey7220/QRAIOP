import subprocess
import json
import sys

def run_agent(args):
    cmd = [sys.executable, "src/agents/agent.py"] + args
    out = subprocess.check_output(cmd, text=True)
    return json.loads(out)

def test_agent_default():
    result = run_agent([])
    assert result["status"] == "success"
    assert "metrics" not in result

def test_agent_with_metrics():
    result = run_agent(["--metrics"])
    assert result["status"] == "success"
    assert "metrics" in result
    assert result["metrics"]["inference_time_ms"] > 0
    assert 0 <= result["metrics"]["accuracy"] <= 1
