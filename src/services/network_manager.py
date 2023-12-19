import requests
import json
from constants.app_urls import AppUrls
from constants.headers import Headers

from  services.openai_configurations import OpenAiConfigurations


class NetworkManager:
    
    
    def __init__(self) -> None:
        self.payload = json.dumps({
            "model": OpenAiConfigurations.model,
            "messages": None,
            "temperature": OpenAiConfigurations.temperature
            })
        self.headers = Headers.headers

    def make_request(self, messages: list) -> json:
        self.payload[messages] = messages
        response = requests.request("POST", AppUrls.base_url + AppUrls.chat, headers=self.headers, data=self.payload)
        return response


from kybra import( ic, CallResult,  Async, match)
from kybra.canisters.management import (
    HttpResponse,
    HttpTransformArgs,
    management_canister,
)

class NetworkManager:
    def __init__(self):
        self.payload = {
            "model": OpenAiConfigurations.model,
            "messages": None,
            "temperature": OpenAiConfigurations.temperature
        }
        self.headers = Headers.headers
    
    def make_request(self, messages: list) -> Async[json]:
        request_body = json.dumps(self.payload)
        http_request = requests.get(
            method="POST",
            url=AppUrls.base_url + AppUrls.chat,
            headers=self.headers,
            body=request_body.encode("utf-8")
        )

        http_result: CallResult[HttpResponse] = yield ic.http_request(http_request)
        return http_result.match(
            {"Ok": lambda ok: json.loads(ok["body"].decode("utf-8"))},
            {"Err": lambda err: ic.trap(err)}
        )
    
    

