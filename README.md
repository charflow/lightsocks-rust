# lightsocks-rust

流量转换

```
SOCKSv5 → encrypt → socket → decrypt → SOCKSv5 → Internet
```

- [ ] Browser -> Client (SOCKSv5)
- [x] Client -> Server (TCP + encryption)
- [ ] Server -> Internet (SOCKSv5)