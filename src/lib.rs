extern crate crypto;

/* An initial test without any data just to see if the idea flies
*/

#[test]
fn it_works() {
  let g = make_gen();
  //This is not what I want! But it turns out not to be the end of the
  // world since evil cannot really do anything with g since there
  // are no primitive operations on Gen which takes a reference.
  //evil(&g);
  let (g1,g2) = split(g);
// If we uncomment the line below we get an error. Just as we want!
//  let (_,_) = split(g);
  let i1 = rand(g1);
// It is possible to rebind generators and mutate them. But
// there is no point in doing that since Gen is abstract.
  let mut g21 = g2;
  let (_,_) = split(g21);
}

pub struct Gen;

pub fn make_gen() -> Gen {
  return Gen;
}

pub fn split(x : Gen) -> (Gen,Gen) {
  return (Gen,Gen);
}

pub fn rand(x : Gen) -> isize {
  return 23;
}

pub fn evil(x : &Gen) {
// In the end there is not much I can do with a function like evil
// because at some point it will have to use split and that can't
// be done
//  let (_,_) = split(x);
}

/* Here is the code that some day might do something properly
*/

/*
struct TFGen {
  key:      [u64; 4],
  level:    u64,
  pos:      u64,
  tree_ix:  i16,
  block_ix: i16,
  block:    [u32; 8],
}

fn split(gen: TFGen) -> (TFGen,TFGen) {

}
*/
/*
mash :: ByteArray# -> Word64 -> Word64 -> Word64 -> Int -> ByteArray
mash k' i b m o32 =
  -- We use unsafeDupablePerformIO here because the cost
  -- of locking in unsafePerformIO is much higher
  -- than any gains it could bring.
  unsafeDupablePerformIO $ do
      (ByteArray c') <- createBlock256 b i m 0
      -- Allocate array for cipher result
      o@(MutableByteArray o') <- newByteArray 32
      threefish256EncryptBlock k' c' o' o32
      unsafeFreezeByteArray o
*/
/*
fn mash(bf: Blowfish, k: [u8], i: u64, b: u64, m: u64, o32: isize) -> [u8] {
  let c = createBlock256(b,i,m,0);
  let o = [0;8];
  encrypt_block(bf,c,o);
  o
}

fn make(key: [u64; 4], level: u64, pos: u64, tree_ix: i16) -> TFGen {
  let bf = init_state();
  return TFGen{key: key, level: level, pos: pos, tree_ix: tree_ix,
               block_ix: 0, block: mash(bf,key,level,pos,0,1)}
}
*/
/*
mkTFGen :: ByteArray -> Word64 -> Word64 -> Int16 -> TFGen
mkTFGen k@(ByteArray k') i b bi =
  TFGen k i b bi 0 (mash k' i b 0 1)
*/

/*
tfGenSplit :: TFGen -> (TFGen, TFGen)
tfGenSplit g@(TFGen k i b bi _ _)
  | bi == maxb = (mkTFGen k' 0 0 1,   mkTFGen k' 0 1   1)
  | otherwise  = (mkTFGen k  i b bi', mkTFGen k  i b'' bi')
  where
  maxb = 64
  bi'  = bi + 1
  k'   = mash' g 0 0
  b''  = setBit b (fromIntegral bi)
*/