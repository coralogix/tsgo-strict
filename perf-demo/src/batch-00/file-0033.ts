import type { Type32 } from '../batch-00/file-0032.js';
export interface Type33 {
  id: 33;
  name: 'File33';
  next: Type32;
}

export function make33(): Type33 {
  return { id: 33, name: 'File33', next: null as unknown as Type32 };
}
