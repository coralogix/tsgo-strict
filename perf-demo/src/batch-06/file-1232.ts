import type { Type1231 } from '../batch-06/file-1231.js';
export interface Type1232 {
  id: 1232;
  name: 'File1232';
  next: Type1231;
}

export function make1232(): Type1232 {
  return { id: 1232, name: 'File1232', next: null as unknown as Type1231 };
}
