import type { Type3130 } from '../batch-15/file-3130.js';
export interface Type3131 {
  id: 3131;
  name: 'File3131';
  next: Type3130;
}

export function make3131(): Type3131 {
  return { id: 3131, name: 'File3131', next: null as unknown as Type3130 };
}
