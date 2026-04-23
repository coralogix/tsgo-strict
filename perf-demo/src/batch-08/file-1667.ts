import type { Type1666 } from '../batch-08/file-1666.js';
export interface Type1667 {
  id: 1667;
  name: 'File1667';
  next: Type1666;
}

export function make1667(): Type1667 {
  return { id: 1667, name: 'File1667', next: null as unknown as Type1666 };
}
