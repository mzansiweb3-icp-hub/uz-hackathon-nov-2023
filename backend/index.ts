import { Canister, query, text, update, None, Opt, ic, nat64, Vec, StableBTreeMap, Record } from 'azle';
import { v4 as uuidv4 } from 'uuid';

const Message = Record({
    id:text,
    email: text,
    language1: text,
    language2: text,
    comment: text,
    language: text,
    topic:text,
  });

  type Message = typeof Message;

  const MessagePayload = Record({
    email: text,
    language1: text,
    language2: text,
    comment: text,
    language: text,
    topic:text,
  });

  type MessagePayload = typeof MessagePayload;

  const Topics = Record({
    id:text,
    topic: text,
  });

  type Topics = typeof Topics;

  const TopicsPayLoad = Record({
    topic: text,
  });

  type TopicsPayLoad = typeof TopicsPayLoad;

  const Languages = Record({
    id:text,
    language: text,
  });

  type Languages = typeof Languages;

  const LanguagesPayLoad = Record({
    language: text,
  });

  type LanguagesPayLoad = typeof LanguagesPayLoad;

  let messageStorage = StableBTreeMap<text, Message>(text, Message, 1);
  let topicsStorage = StableBTreeMap<text, Topics>(text, Topics, 2);
  let languagesStorage = StableBTreeMap<text, Languages>(text, Languages, 3);
  
export default Canister({
    
    getLanguages: query([], Vec(Languages), () => {
        return languagesStorage.values();
      }),

    getMessagesOnLanguage: query([text], Opt(Message), (language) => {
        return messageStorage.get(language);
      }),

    getTopics: query([], Vec(Topics), () => {
        return topicsStorage.values();
      }),

    getMessages: query([], Vec(Message), () => {
        return messageStorage.values();
      }),

    addMessage: update([MessagePayload], Message, (payload) => {
        const message: Message = {
          id: uuidv4(),
          ...payload,
        };
        messageStorage.insert(message.id, message);
        return message;
      }),

      addTopic: update([TopicsPayLoad], Topics, (topicload) => {
        const topic: Topics = {
          id: uuidv4(),
          ...topicload,
        };
        topicsStorage.insert(topic.id, topic);
        return topic;
      }),

      addLanguage: update([LanguagesPayLoad], Languages, (languagepayload) => {
        const language: Languages = {
          id: uuidv4(),
          ...languagepayload,
        };
        languagesStorage.insert(language.id, language);
        return language;
      })
});

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