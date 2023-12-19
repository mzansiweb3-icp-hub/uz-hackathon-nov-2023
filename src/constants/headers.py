import os
class Headers:
    def __init__(self):
        self.headers  = {
            "Content-Type": "application/json",
            "Authorization" : "Bearer " + os.environ.get("OPENAI_KEY")
        }