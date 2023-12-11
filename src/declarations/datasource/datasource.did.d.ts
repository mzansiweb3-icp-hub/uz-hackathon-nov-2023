import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'addLanguage' : ActorMethod<
    [{ 'language' : string }],
    { 'id' : string, 'language' : string }
  >,
  'addMessage' : ActorMethod<
    [
      {
        'topic' : string,
        'language1' : string,
        'language2' : string,
        'email' : string,
        'language' : string,
        'comment' : string,
      },
    ],
    {
      'id' : string,
      'topic' : string,
      'language1' : string,
      'language2' : string,
      'email' : string,
      'language' : string,
      'comment' : string,
    }
  >,
  'addTopic' : ActorMethod<
    [{ 'topic' : string }],
    { 'id' : string, 'topic' : string }
  >,
  'getLanguages' : ActorMethod<
    [],
    Array<{ 'id' : string, 'language' : string }>
  >,
  'getMessages' : ActorMethod<
    [],
    Array<
      {
        'id' : string,
        'topic' : string,
        'language1' : string,
        'language2' : string,
        'email' : string,
        'language' : string,
        'comment' : string,
      }
    >
  >,
  'getMessagesOnLanguage' : ActorMethod<
    [string],
    [] | [
      {
        'id' : string,
        'topic' : string,
        'language1' : string,
        'language2' : string,
        'email' : string,
        'language' : string,
        'comment' : string,
      }
    ]
  >,
  'getTopics' : ActorMethod<[], Array<{ 'id' : string, 'topic' : string }>>,
}
