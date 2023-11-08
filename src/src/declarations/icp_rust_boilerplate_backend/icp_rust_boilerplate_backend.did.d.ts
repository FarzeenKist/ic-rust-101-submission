import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Error = { 'NotFound' : { 'msg' : string } };
export type Result = { 'Ok' : ToDo } |
  { 'Err' : Error };
export interface ToDo {
  'id' : bigint,
  'title' : string,
  'updated_at' : [] | [bigint],
  'body' : string,
  'completed' : boolean,
  'deadline' : bigint,
  'created_at' : bigint,
}
export interface ToDoPayload {
  'title' : string,
  'body' : string,
  'deadline' : bigint,
}
export interface _SERVICE {
  'add_message' : ActorMethod<[ToDoPayload], [] | [ToDo]>,
  'delete_message' : ActorMethod<[bigint], Result>,
  'get_message' : ActorMethod<[bigint], Result>,
  'update_message' : ActorMethod<[bigint, ToDoPayload], Result>,
}
