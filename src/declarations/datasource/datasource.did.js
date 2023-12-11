export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'addLanguage' : IDL.Func(
        [IDL.Record({ 'language' : IDL.Text })],
        [IDL.Record({ 'id' : IDL.Text, 'language' : IDL.Text })],
        [],
      ),
    'addMessage' : IDL.Func(
        [
          IDL.Record({
            'topic' : IDL.Text,
            'language1' : IDL.Text,
            'language2' : IDL.Text,
            'email' : IDL.Text,
            'language' : IDL.Text,
            'comment' : IDL.Text,
          }),
        ],
        [
          IDL.Record({
            'id' : IDL.Text,
            'topic' : IDL.Text,
            'language1' : IDL.Text,
            'language2' : IDL.Text,
            'email' : IDL.Text,
            'language' : IDL.Text,
            'comment' : IDL.Text,
          }),
        ],
        [],
      ),
    'addTopic' : IDL.Func(
        [IDL.Record({ 'topic' : IDL.Text })],
        [IDL.Record({ 'id' : IDL.Text, 'topic' : IDL.Text })],
        [],
      ),
    'getLanguages' : IDL.Func(
        [],
        [IDL.Vec(IDL.Record({ 'id' : IDL.Text, 'language' : IDL.Text }))],
        ['query'],
      ),
    'getMessages' : IDL.Func(
        [],
        [
          IDL.Vec(
            IDL.Record({
              'id' : IDL.Text,
              'topic' : IDL.Text,
              'language1' : IDL.Text,
              'language2' : IDL.Text,
              'email' : IDL.Text,
              'language' : IDL.Text,
              'comment' : IDL.Text,
            })
          ),
        ],
        ['query'],
      ),
    'getMessagesOnLanguage' : IDL.Func(
        [IDL.Text],
        [
          IDL.Opt(
            IDL.Record({
              'id' : IDL.Text,
              'topic' : IDL.Text,
              'language1' : IDL.Text,
              'language2' : IDL.Text,
              'email' : IDL.Text,
              'language' : IDL.Text,
              'comment' : IDL.Text,
            })
          ),
        ],
        ['query'],
      ),
    'getTopics' : IDL.Func(
        [],
        [IDL.Vec(IDL.Record({ 'id' : IDL.Text, 'topic' : IDL.Text }))],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
