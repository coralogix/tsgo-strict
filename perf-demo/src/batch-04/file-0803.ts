import type { Type802 } from '../batch-04/file-0802.js';
export interface Type803 {
  id: 803;
  name: 'File803';
  next: Type802;
}

export function make803(): Type803 {
  return { id: 803, name: 'File803', next: null as unknown as Type802 };
}
