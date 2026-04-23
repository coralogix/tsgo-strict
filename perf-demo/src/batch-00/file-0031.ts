import type { Type30 } from '../batch-00/file-0030.js';
export interface Type31 {
  id: 31;
  name: 'File31';
  next: Type30;
}

export function make31(): Type31 {
  return { id: 31, name: 'File31', next: null as unknown as Type30 };
}
