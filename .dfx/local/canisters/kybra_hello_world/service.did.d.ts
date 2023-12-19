import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'get_message' : ActorMethod<[], string>,
  'get_topic' : ActorMethod<[], string>,
  'set_message' : ActorMethod<[string], undefined>,
  'set_prompt' : ActorMethod<[string, bigint, string, string], undefined>,
}
