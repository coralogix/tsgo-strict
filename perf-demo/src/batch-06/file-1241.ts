import type { Type1240 } from '../batch-06/file-1240.js';
export interface Type1241 {
  id: 1241;
  name: 'File1241';
  next: Type1240;
}

export function make1241(): Type1241 {
  return { id: 1241, name: 'File1241', next: null as unknown as Type1240 };
}
