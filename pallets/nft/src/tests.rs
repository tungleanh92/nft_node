use crate::{mock::*, Error, PayInstallmentOrder};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(PalletNFT::mint_collection(1u64, [1u8; 16], vec!(1), Some(vec!(1)),));

		assert_ok!(PalletNFT::mint(
			1u64,
			[0u8; 16],
			vec!(1),
			Some(vec!(1)),
			vec!(1),
			vec!(1),
			Some(0),
			Some(vec!((5, 1))),
			[1u8; 16]
		));

		assert_ok!(PalletNFT::set_sale_nft(Origin::signed(1), [0u8; 16], 10u128));

		// assert_ok!(PalletNFT::buy_nft(Origin::signed(2), [0u8; 16]));

		assert_ok!(PalletNFT::pay_installment(Origin::signed(2), [0u8; 16], 2, 5u128));
		println!("!!!!!!!!!!!!!!");

		println!("{:?}",PalletNFT::order_by_id([0u8; 16]));

		assert_ok!(PalletNFT::pay_installment(Origin::signed(2), [0u8; 16], 1, 5u128));

		println!("{:?}",PalletNFT::order_by_id([0u8; 16]));

		// assert_ok!(PalletNFT::burn_nft(Origin::signed(2), [0u8; 16]));

		// assert_ok!(PalletNFT::destroy_collection(Origin::signed(1), [1u8; 16]));
	});
}
