import type { Type1095 } from '../batch-05/file-1095.js';
export interface Type1096 {
  id: 1096;
  name: 'File1096';
  next: Type1095;
}

export function make1096(): Type1096 {
  return { id: 1096, name: 'File1096', next: null as unknown as Type1095 };
}
