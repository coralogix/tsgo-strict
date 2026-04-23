import type { Type1180 } from '../batch-05/file-1180.js';
export interface Type1181 {
  id: 1181;
  name: 'File1181';
  next: Type1180;
}

export function make1181(): Type1181 {
  return { id: 1181, name: 'File1181', next: null as unknown as Type1180 };
}
