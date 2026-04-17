import type { Type205 } from '../batch-01/file-0205.js';
export interface Type206 {
  id: 206;
  name: 'File206';
  next: Type205;
}

export function make206(): Type206 {
  return { id: 206, name: 'File206', next: null as unknown as Type205 };
}
