import type { Type1620 } from '../batch-08/file-1620.js';
export interface Type1621 {
  id: 1621;
  name: 'File1621';
  next: Type1620;
}

export function make1621(): Type1621 {
  return { id: 1621, name: 'File1621', next: null as unknown as Type1620 };
}
