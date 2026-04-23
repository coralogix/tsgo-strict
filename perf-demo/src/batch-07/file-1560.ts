import type { Type1559 } from '../batch-07/file-1559.js';
export interface Type1560 {
  id: 1560;
  name: 'File1560';
  next: Type1559;
}

export function make1560(): Type1560 {
  return { id: 1560, name: 'File1560', next: null as unknown as Type1559 };
}
