export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'get_message' : IDL.Func([], [IDL.Text], ['query']),
    'get_topic' : IDL.Func([], [IDL.Text], ['query']),
    'set_message' : IDL.Func([IDL.Text], [], []),
    'set_prompt' : IDL.Func([IDL.Text, IDL.Int, IDL.Text, IDL.Text], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
