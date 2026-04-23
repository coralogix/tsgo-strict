import type { Type1703 } from '../batch-08/file-1703.js';
export interface Type1704 {
  id: 1704;
  name: 'File1704';
  next: Type1703;
}

export function make1704(): Type1704 {
  return { id: 1704, name: 'File1704', next: null as unknown as Type1703 };
}
