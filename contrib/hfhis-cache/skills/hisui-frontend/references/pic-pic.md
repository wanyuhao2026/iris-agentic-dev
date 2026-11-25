# 插画

- 官方页面标题：插画
- 分类：基础
- 来源：http://hisui.cn/api/pic/pic.html
- 官网更新时间：Tue, 12 Mar 2024 03:51:32 GMT
- 提取时间：2026-07-25T13:18:46+00:00

> 此文件由官网演示页自动提取。API 表格和代码来自官网；页面脚本是演示代码，使用时请结合项目现有 HISUI 版本核对。

## 公共插画展示

### 一、患者头像类插画

### 二、院方工作头像类插画

### 三、就诊类型插画

### 四、登录方式插画

### 五、关于插画 切换语言

### 六、系统状态插画 切换语言

```javascript
function init(){
            // --患者--
            var patArr = ['man','woman','unknown-gender'];
            var pre = 'pic-pat-',html=[];
            for(var i=0;i<patArr.length;i++){
                html.push('<div class="conter"><div class="'+pre+patArr[i]+'"/><div class="clstext">'+pre+patArr[i]+'</div></div>');
            }
            $('.conter1').html(html.join(""));
            // --院方人员--
            var usrArr = ['doctor','nurse','surgeon','clothing-worker','medi-worker','dep-director','hosp-director','doctor-woman'];
            pre = 'pic-usr-',html=[];
            for(var i=0;i<usrArr.length;i++){
                html.push('<div class="conter"><div class="'+pre+usrArr[i]+'"/><div class="clstext">'+pre+usrArr[i]+'</div></div>');
            }
            $('.conter2').html(html.join(""));
            // --就诊方式--
            var admArr = ['out','em','in'];
            pre = 'pic-adm-',html=[];
            for(var i=0;i<admArr.length;i++){
                html.push('<div class="conter"><div class="'+pre+admArr[i]+'"/><div class="clstext">'+pre+admArr[i]+'</div></div>');
            }
            $('.conter3').html(html.join(""));
            // --登录方式-
            var logonArr = ['cert','default','face','phone','pin','sound','ukey'];
            pre = 'pic-logon-',html=[];
            for(var i=0;i<logonArr.length;i++){
                html.push('<div class="conter"><div class="'+pre+logonArr[i]+'"/><div class="clstext">'+pre+logonArr[i]+'</div></div>');
            }
            $('.conter4').html(html.join(""));
            // --关于方式-
            var aboutArr = ['dhcc-digitalmed','imedical-logo'];
            pre = 'pic-about-',html=[];
            for(var i=0;i<aboutArr.length;i++){
                html.push('<div class="conter"><div class="'+pre+aboutArr[i]+'"/><div class="clstext">'+pre+aboutArr[i]+'</div></div>');
            }
            $('.conter5').html(html.join(""));
            // -- 系统状态 --
            var arr = ['e403','e404','e500','nodata','nodata-msg','nodata-region','timeout-relogon','welcome'];
            pre = 'pic-sysst-',html=[];
            for(var i=0;i<arr.length;i++){
                html.push('<div class="conter"><div class="'+pre+arr[i]+'"/><div class="clstext">'+pre+arr[i]+'</div></div>');
            }
            $('.conter6').html(html.join(""));
            isChinese = true;
            var linkObj = null;
            $("#langBtn").click(function(){
                if (isChinese){
                    linkObj = $('<link>', {
                        rel: 'stylesheet',
                        type: 'text/css',
                        href: '../../dist/css/locale/hisui.en.css'
                    }).appendTo('head');
                }else{
                    linkObj.remove();
                }
                isChinese = !isChinese;
                $("#langBtn").text('切换成'+(isChinese?'英文':'中文'));
                $("#langBtn").text('切换成'+(isChinese?'英文':'中文'));
            });
            $("#langAboutBtn").click(function(){
                if (isChinese){
                    linkObj = $('<link>', {
                        rel: 'stylesheet',
                        type: 'text/css',
                        href: '../../dist/css/locale/hisui.en.css'
                    }).appendTo('head');
                }else{
                    linkObj.remove();
                }
                isChinese = !isChinese;
                $("#langBtn").text('切换成'+(isChinese?'英文':'中文'));
                $("#langAboutBtn").text('切换成'+(isChinese?'英文':'中文'));
            });
        }
        $(function(){
            init();
        });
```
