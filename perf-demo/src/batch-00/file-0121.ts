import type { Type120 } from '../batch-00/file-0120.js';
export interface Type121 {
  id: 121;
  name: 'File121';
  next: Type120;
}

export function make121(): Type121 {
  return { id: 121, name: 'File121', next: null as unknown as Type120 };
}
