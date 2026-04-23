import type { Type204 } from '../batch-01/file-0204.js';
export interface Type205 {
  id: 205;
  name: 'File205';
  next: Type204;
}

export function make205(): Type205 {
  return { id: 205, name: 'File205', next: null as unknown as Type204 };
}
