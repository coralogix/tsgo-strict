import type { Type340 } from '../batch-01/file-0340.js';
export interface Type341 {
  id: 341;
  name: 'File341';
  next: Type340;
}

export function make341(): Type341 {
  return { id: 341, name: 'File341', next: null as unknown as Type340 };
}
