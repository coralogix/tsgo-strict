import type { Type224 } from '../batch-01/file-0224.js';
export interface Type225 {
  id: 225;
  name: 'File225';
  next: Type224;
}

export function make225(): Type225 {
  return { id: 225, name: 'File225', next: null as unknown as Type224 };
}
