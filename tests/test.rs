#[test]
fn arrary(){
	let v = &[vec![1,2], vec![3,4]];
	v.iter().fold((0u8,0u8),|(i,j),x|{
	let (i1,_j1)=  x.iter().fold((i,j),|(i1,j1),y|{
			println!("x={},y={},v={}", i1,j1, y);
			(i1, j1 + 1)
		});
		(i1+1, 0)
	});
     assert_eq!(v[0][0],1);
}