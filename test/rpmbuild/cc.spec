Summary: CC test package
Name: cc
Version: 1.0
Release: 1
License: GPLv2+
URL: https://www.chinaunicom.cn/
Source: cc.cfg
BuildRequires: bash-completion


%description
CC test package, contains emtyp cc.cfg which installed to /etc

%prep

%build

%install

install -d %{buildroot}%{_sysconfdir}
%check


%files
%defattr(-, root, root)
# config files
%config(noreplace) %{_sysconfdir}/cc.cfg

%changelog
* Wed Feb 08 2023 Jia Chao <jiac13@chinaunicom.cn> 1.0-1
- Init CC.
