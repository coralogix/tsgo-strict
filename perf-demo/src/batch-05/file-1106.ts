import type { Type1105 } from '../batch-05/file-1105.js';
export interface Type1106 {
  id: 1106;
  name: 'File1106';
  next: Type1105;
}

export function make1106(): Type1106 {
  return { id: 1106, name: 'File1106', next: null as unknown as Type1105 };
}
