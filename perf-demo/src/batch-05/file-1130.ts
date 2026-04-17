import type { Type1129 } from '../batch-05/file-1129.js';
export interface Type1130 {
  id: 1130;
  name: 'File1130';
  next: Type1129;
}

export function make1130(): Type1130 {
  return { id: 1130, name: 'File1130', next: null as unknown as Type1129 };
}
