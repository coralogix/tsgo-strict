import type { Type1104 } from '../batch-05/file-1104.js';
export interface Type1105 {
  id: 1105;
  name: 'File1105';
  next: Type1104;
}

export function make1105(): Type1105 {
  return { id: 1105, name: 'File1105', next: null as unknown as Type1104 };
}
