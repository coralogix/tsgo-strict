import type { Type3201 } from '../batch-16/file-3201.js';
export interface Type3202 {
  id: 3202;
  name: 'File3202';
  next: Type3201;
}

export function make3202(): Type3202 {
  return { id: 3202, name: 'File3202', next: null as unknown as Type3201 };
}
