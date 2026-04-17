import type { Type1331 } from '../batch-06/file-1331.js';
export interface Type1332 {
  id: 1332;
  name: 'File1332';
  next: Type1331;
}

export function make1332(): Type1332 {
  return { id: 1332, name: 'File1332', next: null as unknown as Type1331 };
}
