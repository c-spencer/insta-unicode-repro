#[test]
fn unicode() {
    insta::assert_debug_snapshot!(
        "(∀ν : ℕ. (∀α : 🟊. ((α → (1 + 1)) → (Vec ν α → (∃κ : ℕ. Vec κ α)))))",
        @"(∀ν : ℕ. (∀α : ⋆. ((α → (1 + 1)) → (Vec ν α → (∃κ : ℕ. Vec κ α)))))"
    );
}

#[test]
fn unicode_len() {
    assert_eq!("🟊".len(), 4);
    assert_eq!("🟊".chars().count(), 1);

    assert_eq!("(∀ν : ℕ. (∀α : 🟊. ((α → (1 + 1)) → (Vec ν α → (∃κ : ℕ. Vec κ α)))))".chars().count(), 67);
}
