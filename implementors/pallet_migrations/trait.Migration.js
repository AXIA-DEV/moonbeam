(function() {var implementors = {};
implementors["runtime_common"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"allychain_staking/pallet/trait.Config.html\" title=\"trait allychain_staking::pallet::Config\">AllychainStakingConfig</a>&gt; <a class=\"trait\" href=\"pallet_migrations/trait.Migration.html\" title=\"trait pallet_migrations::Migration\">Migration</a> for <a class=\"struct\" href=\"runtime_common/migrations/struct.AllychainStakingManualExits.html\" title=\"struct runtime_common::migrations::AllychainStakingManualExits\">AllychainStakingManualExits</a>&lt;T&gt;","synthetic":false,"types":["runtime_common::migrations::AllychainStakingManualExits"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"allychain_staking/pallet/trait.Config.html\" title=\"trait allychain_staking::pallet::Config\">AllychainStakingConfig</a>&gt; <a class=\"trait\" href=\"pallet_migrations/trait.Migration.html\" title=\"trait pallet_migrations::Migration\">Migration</a> for <a class=\"struct\" href=\"runtime_common/migrations/struct.AllychainStakingPurgeStaleStorage.html\" title=\"struct runtime_common::migrations::AllychainStakingPurgeStaleStorage\">AllychainStakingPurgeStaleStorage</a>&lt;T&gt;","synthetic":false,"types":["runtime_common::migrations::AllychainStakingPurgeStaleStorage"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_author_mapping/pallet/trait.Config.html\" title=\"trait pallet_author_mapping::pallet::Config\">AuthorMappingConfig</a>&gt; <a class=\"trait\" href=\"pallet_migrations/trait.Migration.html\" title=\"trait pallet_migrations::Migration\">Migration</a> for <a class=\"struct\" href=\"runtime_common/migrations/struct.AuthorMappingTwoXToBlake.html\" title=\"struct runtime_common::migrations::AuthorMappingTwoXToBlake\">AuthorMappingTwoXToBlake</a>&lt;T&gt;","synthetic":false,"types":["runtime_common::migrations::AuthorMappingTwoXToBlake"]},{"text":"impl&lt;Runtime, Council, Tech&gt; <a class=\"trait\" href=\"pallet_migrations/trait.Migration.html\" title=\"trait pallet_migrations::Migration\">Migration</a> for <a class=\"struct\" href=\"runtime_common/migrations/struct.MigrateCollectivePallets.html\" title=\"struct runtime_common::migrations::MigrateCollectivePallets\">MigrateCollectivePallets</a>&lt;Runtime, Council, Tech&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Runtime: Config,<br>&nbsp;&nbsp;&nbsp;&nbsp;Council: GetStorageVersion + PalletInfoAccess,<br>&nbsp;&nbsp;&nbsp;&nbsp;Tech: GetStorageVersion + PalletInfoAccess,&nbsp;</span>","synthetic":false,"types":["runtime_common::migrations::MigrateCollectivePallets"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()