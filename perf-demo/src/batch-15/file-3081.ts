import type { Type3080 } from '../batch-15/file-3080.js';
export interface Type3081 {
  id: 3081;
  name: 'File3081';
  next: Type3080;
}

export function make3081(): Type3081 {
  return { id: 3081, name: 'File3081', next: null as unknown as Type3080 };
}
