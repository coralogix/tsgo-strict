import type { Type1225 } from '../batch-06/file-1225.js';
export interface Type1226 {
  id: 1226;
  name: 'File1226';
  next: Type1225;
}

export function make1226(): Type1226 {
  return { id: 1226, name: 'File1226', next: null as unknown as Type1225 };
}
