import type { Type41 } from '../batch-00/file-0041.js';
export interface Type42 {
  id: 42;
  name: 'File42';
  next: Type41;
}

export function make42(): Type42 {
  return { id: 42, name: 'File42', next: null as unknown as Type41 };
}
