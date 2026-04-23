import type { Type701 } from '../batch-03/file-0701.js';
export interface Type702 {
  id: 702;
  name: 'File702';
  next: Type701;
}

export function make702(): Type702 {
  return { id: 702, name: 'File702', next: null as unknown as Type701 };
}
