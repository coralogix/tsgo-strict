import type { Type304 } from '../batch-01/file-0304.js';
export interface Type305 {
  id: 305;
  name: 'File305';
  next: Type304;
}

export function make305(): Type305 {
  return { id: 305, name: 'File305', next: null as unknown as Type304 };
}
