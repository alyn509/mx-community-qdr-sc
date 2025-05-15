from time import sleep
import subprocess

# NOTE: we will make a 6 sec. break between every instruction so that we will not get in the situation of transactions overwriting each other


def execute_with_args(script, arguments=[]):
    script_path = "interaction/" + script + ".py"
    command = ["python3", script_path] + arguments
    result = subprocess.run(command, capture_output=True, text=True)

    sleep(6)
    
    print(result.stdout)
