export const idlFactory = ({ IDL }) => {
  const ToDoPayload = IDL.Record({
    'title' : IDL.Text,
    'body' : IDL.Text,
    'deadline' : IDL.Nat64,
  });
  const ToDo = IDL.Record({
    'id' : IDL.Nat64,
    'title' : IDL.Text,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'body' : IDL.Text,
    'completed' : IDL.Bool,
    'deadline' : IDL.Nat64,
    'created_at' : IDL.Nat64,
  });
  const Error = IDL.Variant({ 'NotFound' : IDL.Record({ 'msg' : IDL.Text }) });
  const Result = IDL.Variant({ 'Ok' : ToDo, 'Err' : Error });
  return IDL.Service({
    'add_message' : IDL.Func([ToDoPayload], [IDL.Opt(ToDo)], []),
    'delete_message' : IDL.Func([IDL.Nat64], [Result], []),
    'get_message' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'update_message' : IDL.Func([IDL.Nat64, ToDoPayload], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
