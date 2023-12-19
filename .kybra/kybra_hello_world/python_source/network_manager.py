# import requests
import json
from constants.app_urls import AppUrls
from constants.headers import Headers

from  services.openai_configurations import OpenAiConfigurations


# class NetworkManager:
    
    
#     def __init__(self) -> None:
#         self.payload = json.dumps({
#             "model": OpenAiConfigurations.model,
#             "messages": None,
#             "temperature": OpenAiConfigurations.temperature
#             })
#         self.headers = Headers.headers

#     def make_request(self, messages: list) -> json:
#         self.payload[messages] = messages
#         response = requests.request("POST", AppUrls.base_url + AppUrls.chat, headers=self.headers, data=self.payload)
#         return response


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
    
    # @update
    # def make_request(self, messages: list) -> Async[json]:
    #     request_body = json.dumps(self.payload)
    #     http_request = HttpRequest(
    #         method="POST",
    #         url=AppUrls.base_url + AppUrls.chat,
    #         headers=self.headers,
    #         body=request_body.encode("utf-8")
    #     )

    #     http_result: CallResult[HttpResponse] = yield ic.http_request(http_request)
    #     return http_result.match(
    #         {"Ok": lambda ok: json.loads(ok["body"].decode("utf-8"))},
    #         {"Err": lambda err: ic.trap(err)}
    #     )
    
    
    def eth_get_balance(self,ethereum_address: str) -> Async[json]:
        request_body = json.dumps(self.payload)
        http_result: CallResult[HttpResponse] = yield management_canister.http_request(
            {
                "url": AppUrls.base_url + AppUrls.chat,
                "max_response_bytes": 2_000,
                "method": {"post": None},
                "headers": self.headers,
                "body": request_body.encode("utf-8"),
               # "transform": {"function": (ic.id(), "eth_transform"), "context": bytes()},
            }
        ).with_cycles(50_000_000)

        return match(
            http_result,
            {"Ok": lambda ok: ok["body"].decode("utf-8"), "Err": lambda err: ic.trap(err)},
        )
