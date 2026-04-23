import type { Type201 } from '../batch-01/file-0201.js';
export interface Type202 {
  id: 202;
  name: 'File202';
  next: Type201;
}

export function make202(): Type202 {
  return { id: 202, name: 'File202', next: null as unknown as Type201 };
}
