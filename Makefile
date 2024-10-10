include $(TOPDIR)/rules.mk

PKG_NAME:=ua4f
PKG_VERSION:=0.1.0
PKG_MAINTAINER := James Clef <qwq233@qwq2333.top>

PKG_LICENSE:=AGPL-3.0-or-later
PKG_LICENSE_FILES:=LICENSE.md

HOST_BUILD_DEPENDS:=rust/host

include $(INCLUDE_DIR)/package.mk
include ./rustc_targets.mk

define Package/ua4f
  SECTION:=net
  CATEGORY:=Network
  TITLE:=UA4F
endef

define Package/ua4f/description
  Another User Agent faker, allowing users to bypass multi device detection for Campus Network via socks5 proxy.
endef
	
define Build/Prepare
	$(call Build/Prepare/Default)
	mkdir -p $(PKG_BUILD_DIR)
	$(CP) ./* $(PKG_BUILD_DIR)/
	sed -i 's/0.1.0/$(PKG_VERSION)/g' $(PKG_BUILD_DIR)/Cargo.toml
	sed -i 's/0.1.0/$(PKG_VERSION)/g' $(PKG_BUILD_DIR)/openwrt/usr/lib/lua/luci/controller/ua4f.lua
endef

define Build/Compile
	cd $(PKG_BUILD_DIR) && cargo rustc --target=$(RUSTC_TARGET_ARCH) --release -- -C linker=$(TARGET_CC_NOCACHE) -C ar=$(TARGET_AR)
	stat $(PKG_BUILD_DIR)/target/$(RUSTC_TARGET_ARCH)/release/ua4f
endef

define Package/ua4f/install
	$(INSTALL_DIR) $(1)/usr/bin
	$(INSTALL_BIN) $(PKG_BUILD_DIR)/target/$(RUSTC_TARGET_ARCH)/release/ua4f $(1)/usr/bin/ua4f

	$(INSTALL_DIR) $(1)/etc/config
	$(INSTALL_BIN) $(PKG_BUILD_DIR)/openwrt/etc/config/ua4f $(1)/etc/config/ua4f

	$(INSTALL_DIR) $(1)/etc/init.d
	$(INSTALL_BIN) $(PKG_BUILD_DIR)/openwrt/etc/init.d/ua4f $(1)/etc/init.d/ua4f

	$(INSTALL_DIR) $(1)/usr/lib/lua/luci/controller/
	$(INSTALL_BIN) $(PKG_BUILD_DIR)/openwrt/usr/lib/lua/luci/controller/ua4f.lua $(1)/usr/lib/lua/luci/controller/ua4f.lua
	$(INSTALL_DIR) $(1)/usr/lib/lua/luci/model/cbi/
	$(INSTALL_BIN) $(PKG_BUILD_DIR)/openwrt/usr/lib/lua/luci/model/cbi/ua4f.lua $(1)/usr/lib/lua/luci/model/cbi/ua4f.lua
endef

$(eval $(call BuildPackage,ua4f))