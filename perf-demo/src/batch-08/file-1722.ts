import type { Type1721 } from '../batch-08/file-1721.js';
export interface Type1722 {
  id: 1722;
  name: 'File1722';
  next: Type1721;
}

export function make1722(): Type1722 {
  return { id: 1722, name: 'File1722', next: null as unknown as Type1721 };
}
