import {
  query,
  update,
  Record,
  StableBTreeMap,
  Vec,
  nat64,
  ic,
  Opt,
  None,
  text,
  Canister,
  Principal,
} from "azle";
import { v4 as uuidv4 } from "uuid";


const Message = Record({
  id: text,
  title: text,
  body: text,
  attachmentURL: text,
  createdAt: nat64,
  updatedAt: Opt(nat64),
});

type Message = typeof Message;

const MessagePayload = Record({
  title: text,
  body: text,
  attachmentURL: text,
});
type MessagePayload = typeof MessagePayload;

let messageStorage = StableBTreeMap<text, Message>(text, Message, 0);

export default Canister({
  // $query;
  // export function getMessages(): Result<Vec<Message>, string> {
  //     return Result.Ok(messageStorage.values());
  // }

  getMessages: query([], Vec(Message), () => {
    return messageStorage.values();
  }),

  // $query;
  // export function getMessage(id: string): Result<Message, string> {
  //     return match(messageStorage.get(id), {
  //         Some: (message) => Result.Ok<Message, string>(message),
  //         None: () => Result.Err<Message, string>(`a message with id=${id} not found`)
  //     });
  // }

  getMessage: query([text], Opt(Message), (id) => {
    return messageStorage.get(id);
  }),

  // $update;
  // export function addMessage(payload: MessagePayload): Result<Message, string> {
  //     const message: Message = { id: uuidv4(), createdAt: ic.time(), updatedAt: Opt.None, ...payload };
  //     messageStorage.insert(message.id, message);
  //     return Result.Ok(message);
  // }
  
  addMessage: update([MessagePayload], Message, (payload) => {
    const message: Message = {
      id: uuidv4(),
      createdAt: ic.time(),
      updatedAt: None,
      ...payload,
    };
    messageStorage.insert(message.id, message);
    return message;
  }),

  // $update;
  // export function deleteMessage(id: string): Result<Message, string> {
  //     return match(messageStorage.remove(id), {
  //         Some: (deletedMessage) => Result.Ok<Message, string>(deletedMessage),
  //         None: () => Result.Err<Message, string>(`couldn't delete a message with id=${id}. message not found.`)
  //     });
  // }

  deleteMessage: update([text], Opt(Message), (id) => {
    return messageStorage.remove(id);
  }),
});

// a workaround to make uuid package work with Azle
globalThis.crypto = {
  // @ts-ignore
 getRandomValues: () => {
     let array = new Uint8Array(32)

     for (let i = 0; i < array.length; i++) {
         array[i] = Math.floor(Math.random() * 256)
     }

     return array
 }
}
