import type { Type303 } from '../batch-01/file-0303.js';
export interface Type304 {
  id: 304;
  name: 'File304';
  next: Type303;
}

export function make304(): Type304 {
  return { id: 304, name: 'File304', next: null as unknown as Type303 };
}
