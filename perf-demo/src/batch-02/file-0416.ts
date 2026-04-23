import type { Type415 } from '../batch-02/file-0415.js';
export interface Type416 {
  id: 416;
  name: 'File416';
  next: Type415;
}

export function make416(): Type416 {
  return { id: 416, name: 'File416', next: null as unknown as Type415 };
}
