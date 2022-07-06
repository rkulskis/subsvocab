use std::fs::File;
use std::io::Read;
use scraper::{Html, Selector};

pub fn parse_subs(filename: &str) -> Vec<&str> {
    // load in subs file and read to string
    let mut file = File::open(filename).expect("File not found");
    let mut data = String::new();
    let mut list: Vec<&str> = Vec::new();
    file.read_to_string(&mut data).expect("Error reading file");

    // TODO: string/slicing better conversion
    let fragment = Html::parse_fragment(r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
    <tt xmlns:tt="http://www.w3.org/ns/ttml" xmlns:ttm="http://www.w3.org/ns/ttml#metadata" xmlns:ttp="http://www.w3.org/ns/ttml#parameter" xmlns:tts="http://www.w3.org/ns/ttml#styling" ttp:tickRate="10000000" ttp:timeBase="media" xmlns="http://www.w3.org/ns/ttml">
    <head>
    <ttp:profile use="http://netflix.com/ttml/profile/dfxp-ls-sdh"/>
    <styling>
    <style tts:textAlign="center" xml:id="style_0"/>
    <style tts:color="magenta" xml:id="style_1"/>
    <style tts:color="white" xml:id="style_2"/>
    <style tts:textAlign="right" xml:id="style_3"/>
    <style tts:color="yellow" xml:id="style_4"/>
    <style tts:color="cyan" xml:id="style_5"/>
    <style tts:color="green" xml:id="style_6"/>
    </styling>
    <layout>
    <region xml:id="Region_00">
    <style tts:textAlign="center"/>
    <style tts:displayAlign="after"/>
    <style tts:origin="10.00% 79.34%"/>
    <style tts:extent="80.00% 10.66%"/>
    </region>
    <region xml:id="Region_01">
    <style tts:textAlign="center"/>
    <style tts:displayAlign="after"/>
    <style tts:origin="10.00% 84.67%"/>
    <style tts:extent="80.00% 5.33%"/>
    </region>
    <region xml:id="Region_02">
    <style tts:textAlign="right"/>
    <style tts:displayAlign="before"/>
    <style tts:origin="10.00% 10.00%"/>
    <style tts:extent="80.00% 5.33%"/>
    </region>
    <region xml:id="Region_03">
    <style tts:textAlign="center"/>
    <style tts:displayAlign="before"/>
    <style tts:origin="10.00% 10.00%"/>
    <style tts:extent="80.00% 5.33%"/>
    </region>
    <region xml:id="Region_04">
    <style tts:textAlign="center"/>
    <style tts:displayAlign="before"/>
    <style tts:origin="10.00% 10.00%"/>
    <style tts:extent="80.00% 10.66%"/>
    </region>
    <region xml:id="Region_05">
    <style tts:textAlign="right"/>
    <style tts:displayAlign="after"/>
    <style tts:origin="10.00% 84.67%"/>
    <style tts:extent="80.00% 5.33%"/>
    </region>
    </layout>
    </head>
    <body>
    <div xml:space="preserve">
    <p begin="60000000t" end="81600000t" region="Region_00" xml:id="subtitle0"><span style="style_1">"Mucho cuidado,</span><br/><span style="style_1">porque en el momento en que haya</span></p>
    <p begin="82400000t" end="106000000t" region="Region_00" xml:id="subtitle1"><span style="style_1">una sola gota de sangre,</span><br/><span style="style_1">dejaremos de ser unos Robin Hood</span></p>
    <p begin="106800000t" end="130000000t" region="Region_00" xml:id="subtitle2"><span style="style_1">para convertirnos simplemente</span><br/><span style="style_1">en unos hijos de puta".</span></p>
    <p begin="130800000t" end="138800000t" region="Region_01" xml:id="subtitle3"><span style="style_2">(GRITAN)</span></p>
    <p begin="139600000t" end="157600000t" region="Region_02" xml:id="subtitle4"><span style="style_2">(Ráfaga de disparos)</span></p>
    <p begin="209200000t" end="223200000t" region="Region_01" xml:id="subtitle5"><span style="style_4">¡Río, Río!</span></p>
    <p begin="237600000t" end="244800000t" region="Region_02" xml:id="subtitle6"><span style="style_2">(Disparos)</span></p>
    <p begin="291200000t" end="305600000t" region="Region_01" xml:id="subtitle7"><span style="style_2">¡Cago en la puta, tío!</span></p>
    <p begin="306400000t" end="334000000t" region="Region_02" xml:id="subtitle8"><span style="style_2">(Disparos)</span></p>
    <p begin="377600000t" end="404400000t" region="Region_02" xml:id="subtitle9"><span style="style_2">(Disparos)</span></p>
    <p begin="646000000t" end="664400000t" region="Region_01" xml:id="subtitle10"><span style="style_4">(HABLA ENTRE SOLLOZOS)</span></p>
    <p begin="665200000t" end="683600000t" region="Region_01" xml:id="subtitle11"><span style="style_2">¡Me cago en la puta, Tokio!</span></p>
    <p begin="684400000t" end="712400000t" region="Region_00" xml:id="subtitle12"><span style="style_2">¡La primera en la frente, tío,</span><br/><span style="style_2">la puta primera en la frente!</span></p>
    <p begin="713200000t" end="722400000t" region="Region_01" xml:id="subtitle13"><span style="style_2">¡La hostia!</span></p>
    <p begin="754400000t" end="766000000t" region="Region_01" xml:id="subtitle14"><span style="style_2">¡Joder!</span></p>
    <p begin="786800000t" end="797200000t" region="Region_01" xml:id="subtitle15"><span style="style_2">A ver...</span></p>
    <p begin="806800000t" end="834800000t" region="Region_00" xml:id="subtitle16"><span style="style_2">Paula,</span><br/><span style="style_2">te falta una flor morada ahí.</span></p>
    <p begin="844000000t" end="852400000t" region="Region_01" xml:id="subtitle17"><span style="style_2">Paula.</span></p>
    <p begin="860800000t" end="870400000t" region="Region_01" xml:id="subtitle18"><span style="style_2">Paula.</span></p>
    <p begin="878400000t" end="907600000t" region="Region_00" xml:id="subtitle19"><span style="style_2">-Es que no entiendo</span><br/><span style="style_2">por qué papá no puede venir mañana</span></p>
    <p begin="908400000t" end="944000000t" region="Region_00" xml:id="subtitle20"><span style="style_2">a mi cumpleaños.</span><br/><span style="style_2">-Pues ya te lo he explicado, Paula.</span></p>
    <p begin="944800000t" end="968000000t" region="Region_00" xml:id="subtitle21"><span style="style_2">Un juez le ha quitado la custodia</span><br/><span style="style_2">y ahora voy a ser yo</span></p>
    <p begin="968800000t" end="990400000t" region="Region_00" xml:id="subtitle22"><span style="style_2">la que te va a cuidar.</span><br/><span style="style_2">-No ha sido un juez.</span></p>
    <p begin="991200000t" end="1003600000t" region="Region_00" xml:id="subtitle23"><span style="style_2">-Ah, ¿no?</span><br/><span style="style_2">-No.</span></p>
    <p begin="1004400000t" end="1015200000t" region="Region_01" xml:id="subtitle24"><span style="style_2">-¿Y quién ha sido?</span></p>
    <p begin="1016000000t" end="1043200000t" region="Region_00" xml:id="subtitle25"><span style="style_2">-Has sido tú, que no quieres</span><br/><span style="style_2">que venga a mi cumpleaños.</span></p>
    <p begin="1047600000t" end="1060000000t" region="Region_01" xml:id="subtitle26"><span style="style_2">¿A que sí, abuela?</span></p>
    <p begin="1060800000t" end="1097600000t" region="Region_00" xml:id="subtitle27"><span style="style_2">-Eh... Bueno, verás,</span><br/><span style="style_2">papá y mamá ahora están regañados.</span></p>
    <p begin="1110400000t" end="1134400000t" region="Region_00" xml:id="subtitle28"><span style="style_2">Ya verás como se arreglan.</span><br/><span style="style_2">-Mamá, por favor.</span></p>
    <p begin="1135200000t" end="1155600000t" region="Region_02" xml:id="subtitle29"><span style="style_2">(Teléfono)</span></p>
    <p begin="1200000000t" end="1210400000t" region="Region_01" xml:id="subtitle30"><span style="style_2">Comisario.</span></p>
    <p begin="1219200000t" end="1269200000t" region="Region_00" xml:id="subtitle31"><span style="style_2">-"Raquel, sé que es tu día libre,</span><br/><span style="style_2">pero tenemos un atraco con rehenes.</span></p>
    <p begin="1270000000t" end="1303600000t" region="Region_00" xml:id="subtitle32"><span style="style_2">Los atracadores han intentado salir</span><br/><span style="style_2">con el dinero y no han podido.</span></p>
    <p begin="1304400000t" end="1319200000t" region="Region_01" xml:id="subtitle33"><span style="style_2">Han herido a dos policías.</span></p>
    <p begin="1325600000t" end="1348800000t" region="Region_00" xml:id="subtitle34"><span style="style_2">Es en la Fábrica Nacional</span><br/><span style="style_2">de Moneda y Timbre.</span></p>
    <p begin="1349600000t" end="1368400000t" region="Region_00" xml:id="subtitle35"><span style="style_2">Quiero que lleves tú</span><br/><span style="style_2">la negociación".</span></p>
    <p begin="1428000000t" end="1456800000t" region="Region_00" xml:id="subtitle36"><span style="style_2">-Mándame un coche.</span><br/><span style="style_2">-"Lo tienes en la puerta".</span></p>
    <p begin="1502800000t" end="1529200000t" region="Region_00" xml:id="subtitle37"><span style="style_2">(DENVER) ¡Explícame</span><br/><span style="style_2">qué puta mierda ha sido eso!</span></p>
    <p begin="1530000000t" end="1558000000t" region="Region_00" xml:id="subtitle38"><span style="style_2">¡Explícame qué cojones ha sido eso!</span><br/><span style="style_2">¡Se te va la cabeza!</span></p>
    <p begin="1588000000t" end="1610400000t" region="Region_00" xml:id="subtitle39"><span style="style_2">¡Has acribillado a dos policías!</span><br/><span style="style_4">Tranquilízate.</span></p>
    <p begin="1611200000t" end="1641600000t" region="Region_00" xml:id="subtitle40"><span style="style_4">Apareció un puto policía</span><br/><span style="style_4">disparándome.</span></p>
    <p begin="1642400000t" end="1658800000t" region="Region_01" xml:id="subtitle41"><span style="style_4">¿Qué hubieras hecho tú, escupirle?</span></p>
    <p begin="1659600000t" end="1686400000t" region="Region_00" xml:id="subtitle42"><span style="style_2">Mira, ¡me cago en mi vida!</span><br/><span style="style_2">-Pues qué vas a hacer, tía,</span></p>
    <p begin="1687200000t" end="1709600000t" region="Region_00" xml:id="subtitle43"><span style="style_2">qué vas a hacer,</span><br/><span style="style_2">seguir el puto plan, joder,</span></p>
    <p begin="1710400000t" end="1732000000t" region="Region_00" xml:id="subtitle44"><span style="style_2">que lo hemos repasado</span><br/><span style="style_2">400 millones de veces.</span></p>
    <p begin="1756400000t" end="1781200000t" region="Region_00" xml:id="subtitle45"><span style="style_2">Lo hemos dicho, no íbamos</span><br/><span style="style_2">a disparar a nadie.</span></p>
    <p begin="1784000000t" end="1814800000t" region="Region_00" xml:id="subtitle46"><span style="style_2">-Caí bloqueado con los impactos...</span><br/><span style="style_2">-¡Cállate!</span></p>
    <p begin="1816800000t" end="1844000000t" region="Region_00" xml:id="subtitle47"><span style="style_2">Que también te cubrimos,</span><br/><span style="style_2">pero disparando al suelo,</span></p>
    <p begin="1844800000t" end="1864400000t" region="Region_01" xml:id="subtitle48"><span style="style_2">no a los cuerpos a quemarropa.</span></p>
    <p begin="1865200000t" end="1878800000t" region="Region_02" xml:id="subtitle49"><span style="style_2">(Puerta)</span></p>
    <p begin="1939600000t" end="1962800000t" region="Region_00" xml:id="subtitle50"><span style="style_2">(BERLÍN) Ya se están llevando</span><br/><span style="style_2">a los policías heridos.</span></p>
    <p begin="1972000000t" end="1990800000t" region="Region_01" xml:id="subtitle51"><span style="style_2">¿Están conectados los teléfonos?</span></p>
    <p begin="2012400000t" end="2039200000t" region="Region_00" xml:id="subtitle52"><span style="style_2">Fuera cualquier señal inalámbrica</span><br/><span style="style_2">o de radio.</span></p>
    <p begin="2060400000t" end="2079600000t" region="Region_01" xml:id="subtitle53"><span style="style_2">Pasamos a analógico.</span></p>
    <p begin="2176000000t" end="2192400000t" region="Region_01" xml:id="subtitle54"><span style="style_2">Llama al Profesor.</span></p>
    <p begin="2270400000t" end="2287600000t" region="Region_01" xml:id="subtitle55"><span style="style_1">¿Qué lleva puesto?</span></p>
    <p begin="2305600000t" end="2323200000t" region="Region_01" xml:id="subtitle56"><span style="style_1">¿Qué lleva puesto?</span></p>
    <p begin="2340000000t" end="2354800000t" region="Region_01" xml:id="subtitle57"><span style="style_1">¿Qué lleva puesto?</span></p>
    <p begin="2355600000t" end="2375200000t" region="Region_02" xml:id="subtitle58"><span style="style_2">(Teléfono)</span></p>
    <p begin="2414000000t" end="2441600000t" region="Region_00" xml:id="subtitle59"><span style="style_1">¿Qué ha pasado?</span><br/><span style="style_2">(BERLÍN) "Dos policías heridos".</span></p>
    <p begin="2447600000t" end="2462000000t" region="Region_01" xml:id="subtitle60"><span style="style_1">¿Quién ha disparado?</span></p>
    <p begin="2467200000t" end="2478000000t" region="Region_03" xml:id="subtitle61"><span style="style_2">"Tokio".</span></p>
    <p begin="2489200000t" end="2509200000t" region="Region_03" xml:id="subtitle62"><span style="style_2">Conecta las cámaras al Profesor.</span></p>
    <p begin="2513600000t" end="2535600000t" region="Region_03" xml:id="subtitle63"><span style="style_2">"Rozaron a Río y Tokio disparó".</span></p>
    <p begin="2551600000t" end="2570400000t" region="Region_03" xml:id="subtitle64"><span style="style_2">Al parecer, tienen una relación.</span></p>
    <p begin="2594000000t" end="2604000000t" region="Region_03" xml:id="subtitle65"><span style="style_1">Pásamela.</span></p>
    <p begin="2710800000t" end="2728000000t" region="Region_04" xml:id="subtitle66"><span style="style_4">"¿Qué?".</span><br/><span style="style_1">¿Es cierto eso,</span></p>
    <p begin="2734800000t" end="2764800000t" region="Region_04" xml:id="subtitle67"><span style="style_1">que tienes una relación con Río?</span><br/><span style="style_4">¿Qué coño estás diciendo?</span></p>
    <p begin="2775600000t" end="2784400000t" region="Region_03" xml:id="subtitle68"><span style="style_4">No.</span></p>
    <p begin="2789200000t" end="2806400000t" region="Region_04" xml:id="subtitle69"><span style="style_4">El amor de mi vida</span><br/><span style="style_4">murió por mi culpa,</span></p>
    <p begin="2807200000t" end="2832800000t" region="Region_04" xml:id="subtitle70"><span style="style_4">en lo último que pienso es</span><br/><span style="style_4">en tener una relación con un crío.</span></p>
    <p begin="2833600000t" end="2866400000t" region="Region_04" xml:id="subtitle71"><span style="style_4">"Disparé para protegerme;</span><br/><span style="style_4">a mí y a mi compañero".</span></p>
    <p begin="2867200000t" end="2902400000t" region="Region_04" xml:id="subtitle72"><span style="style_4">Y, señor Profesor, por mucho</span><br/><span style="style_4">que lo haya pensado, las cosas</span></p>
    <p begin="2903200000t" end="2926400000t" region="Region_04" xml:id="subtitle73"><span style="style_4">"no siempre salen</span><br/><span style="style_4">como las tiene previstas".</span></p>
    <p begin="3698800000t" end="3718000000t" region="Region_04" xml:id="subtitle74"><span style="style_2">¿Qué ha pasado?</span><br/><span style="style_2">-Han reventado una cámara</span></p>
    <p begin="3718800000t" end="3754800000t" region="Region_04" xml:id="subtitle75"><span style="style_2">de la Fábrica de Moneda</span><br/><span style="style_2">para llevarse las últimas planchas.</span></p>
    <p begin="3756000000t" end="3786400000t" region="Region_04" xml:id="subtitle76"><span style="style_2">-¿Por qué no saltaron las alarmas?</span><br/><span style="style_2">-Ha saltado una alarma manual,</span></p>
    <p begin="3787200000t" end="3813600000t" region="Region_04" xml:id="subtitle77"><span style="style_2">suponemos que fue algún trabajador.</span><br/><span style="style_2">-¿Cuántos rehenes?</span></p>
    <p begin="3814400000t" end="3839600000t" region="Region_04" xml:id="subtitle78"><span style="style_2">-Imposible saberlo,</span><br/><span style="style_2">el museo estaba abierto.</span></p>
    <p begin="3840400000t" end="3850000000t" region="Region_03" xml:id="subtitle79"><span style="style_2">-Dame un número.</span></p>
    <p begin="3850800000t" end="3880000000t" region="Region_04" xml:id="subtitle80"><span style="style_2">-Confirmados: 35 trabajadores,</span><br/><span style="style_2">11 guardias de seguridad</span></p>
    <p begin="3880800000t" end="3900000000t" region="Region_04" xml:id="subtitle81"><span style="style_2">y 17 chavales</span><br/><span style="style_2">del Colegio Británico.</span></p>
    <p begin="3900800000t" end="3924800000t" region="Region_04" xml:id="subtitle82"><span style="style_2">-¿Qué coño hacían ahí?</span><br/><span style="style_2">-Estaban de excursión.</span></p>
    <p begin="3925600000t" end="3949200000t" region="Region_00" xml:id="subtitle83"><span style="style_2">-¿Ha trascendido a la prensa?</span><br/><span style="style_2">-No, no, aún no.</span></p>
    <p begin="3958400000t" end="3976000000t" region="Region_00" xml:id="subtitle84"><span style="style_2">-¿Atracadores?</span><br/><span style="style_2">-No lo sabemos aún,</span></p>
    <p begin="3976800000t" end="4004000000t" region="Region_00" xml:id="subtitle85"><span style="style_2">estamos recopilando imágenes</span><br/><span style="style_2">de los comercios cercanos.</span></p>
    <p begin="4004800000t" end="4030000000t" region="Region_00" xml:id="subtitle86"><span style="style_2">(MÉDICO) Ha perdido mucha sangre.</span><br/><span style="style_2">-¿Cuántos puede haber?</span></p>
    <p begin="4030800000t" end="4058000000t" region="Region_00" xml:id="subtitle87"><span style="style_2">-Han salido cuatro,</span><br/><span style="style_2">creemos que no menos de seis.</span></p>
    <p begin="4061200000t" end="4071600000t" region="Region_01" xml:id="subtitle88"><span style="style_2">-¿Armas?</span></p>
    <p begin="4072400000t" end="4092400000t" region="Region_00" xml:id="subtitle89"><span style="style_2">-Sabemos que tienen</span><br/><span style="style_2">al menos tres M16.</span></p>
    <p begin="4093200000t" end="4121200000t" region="Region_00" xml:id="subtitle90"><span style="style_2">Y son extremadamente agresivos.</span><br/><span style="style_2">-Ya están aquí los GEO.</span></p>
    <p begin="4122000000t" end="4143200000t" region="Region_00" xml:id="subtitle91"><span style="style_2">-Sí, han sido</span><br/><span style="style_2">los primeros en llegar.</span></p>
    <p begin="4161600000t" end="4180800000t" region="Region_00" xml:id="subtitle92"><span style="style_2">-Por favor,</span><br/><span style="style_2">dame alguna buena noticia.</span></p>
    <p begin="4190800000t" end="4230000000t" region="Region_00" xml:id="subtitle93"><span style="style_2">-Los policías</span><br/><span style="style_2">a los que han disparado están vivos.</span></p>
    <p begin="4249200000t" end="4260000000t" region="Region_01" xml:id="subtitle94"><span style="style_2">-Menos mal.</span></p>
    <p begin="4283600000t" end="4294800000t" region="Region_01" xml:id="subtitle95"><span style="style_2">Vamos.</span></p>
    <p begin="4351600000t" end="4374800000t" region="Region_00" xml:id="subtitle96"><span style="style_2">¿Qué pasa con las cámaras</span><br/><span style="style_2">de dentro del edificio?</span></p>
    <p begin="4375600000t" end="4393200000t" region="Region_00" xml:id="subtitle97"><span style="style_2">¿Dónde está la señal?</span><br/><span style="style_2">-No hay señal.</span></p>
    <p begin="4394000000t" end="4410400000t" region="Region_00" xml:id="subtitle98"><span style="style_2">-¿Cómo que no?</span><br/><span style="style_2">-No hay señal, no sé.</span></p>
    <p begin="4411200000t" end="4441200000t" region="Region_00" xml:id="subtitle99"><span style="style_2">Deben haberla roto o desviado.</span><br/><span style="style_2">Estamos trabajando en ello.</span></p>
    <p begin="4442000000t" end="4452000000t" region="Region_01" xml:id="subtitle100"><span style="style_2">-¿Qué pasa con la red?</span></p>
    <p begin="4452800000t" end="4482000000t" region="Region_00" xml:id="subtitle101"><span style="style_2">-Han tumbado la fibra óptica,</span><br/><span style="style_2">no podemos hackearlos.</span></p>
    <p begin="4501200000t" end="4536000000t" region="Region_00" xml:id="subtitle102"><span style="style_2">-Escuchadme bien todos,</span><br/><span style="style_2">Policía, Protección Civil,</span></p>
    <p begin="4536800000t" end="4561600000t" region="Region_00" xml:id="subtitle103"><span style="style_2">quiero</span><br/><span style="style_2">todos los teléfonos desconectados.</span></p>
    <p begin="4562400000t" end="4589600000t" region="Region_00" xml:id="subtitle104"><span style="style_2">Vamos a liberar el repetidor</span><br/><span style="style_2">situado en el tejado del edificio</span></p>
    <p begin="4590400000t" end="4604800000t" region="Region_01" xml:id="subtitle105"><span style="style_2">por si hacen alguna llamada.</span></p>
    <p begin="4605600000t" end="4626400000t" region="Region_00" xml:id="subtitle106"><span style="style_2">Cualquier señal de móvil</span><br/><span style="style_2">de todo el perímetro</span></p>
    <p begin="4627200000t" end="4651200000t" region="Region_00" xml:id="subtitle107"><span style="style_2">quiero que entre</span><br/><span style="style_2">directamente por aquí.</span></p>
    <p begin="4652000000t" end="4678800000t" region="Region_00" xml:id="subtitle108"><span style="style_2">-Vale, quiero</span><br/><span style="style_2">a los analistas de escucha ya aquí,</span></p>
    <p begin="4679600000t" end="4698800000t" region="Region_01" xml:id="subtitle109"><span style="style_2">en dos segundos. ¿De acuerdo?</span></p>
    <p begin="4699600000t" end="4720000000t" region="Region_00" xml:id="subtitle110"><span style="style_2">-¿Tenemos los planos</span><br/><span style="style_2">de dentro del edificio?</span></p>
    <p begin="4720800000t" end="4738000000t" region="Region_01" xml:id="subtitle111"><span style="style_2">-Estudio una posible intervención.</span></p>
    <p begin="4804000000t" end="4825200000t" region="Region_01" xml:id="subtitle112"><span style="style_2">"Volluto" intenso.</span></p>
    <p begin="4846800000t" end="4874800000t" region="Region_00" xml:id="subtitle113"><span style="style_2">Luego dicen</span><br/><span style="style_2">que el funcionariado no se cuida.</span></p>
    <p begin="4882000000t" end="4894400000t" region="Region_01" xml:id="subtitle114"><span style="style_2">-¿Qué coño haces?</span></p>
    <p begin="4905200000t" end="4921200000t" region="Region_01" xml:id="subtitle115"><span style="style_2">(BERLÍN) Es nuestro ratito libre.</span></p>
    <p begin="4926000000t" end="4966800000t" region="Region_00" xml:id="subtitle116"><span style="style_2">Tienen que montar el campamento,</span><br/><span style="style_2">enviarnos un dron,</span></p>
    <p begin="4967600000t" end="4982400000t" region="Region_01" xml:id="subtitle117"><span style="style_2">buscar los planos del edificio;</span></p>
    <p begin="4983200000t" end="5013600000t" region="Region_00" xml:id="subtitle118"><span style="style_2">hay que darles un poquito de tiempo</span><br/><span style="style_2">para que se organicen.</span></p>
    <p begin="5031200000t" end="5070800000t" region="Region_00" xml:id="subtitle119"><span style="style_2">Oye, ¿por qué Tokio</span><br/><span style="style_2">habrá dicho que no estáis juntos?</span></p>
    <p begin="5077600000t" end="5093200000t" region="Region_01" xml:id="subtitle120"><span style="style_2">-Porque no lo estamos.</span></p>
    <p begin="5112800000t" end="5140400000t" region="Region_00" xml:id="subtitle121"><span style="style_2">-¿Y por qué habré escuchado</span><br/><span style="style_2">cada noche el cabecero de su cama</span></p>
    <p begin="5141200000t" end="5170400000t" region="Region_00" xml:id="subtitle122"><span style="style_2">como un martillo percutor?</span><br/><span style="style_2">¿Estará aprendiendo a bailar samba</span></p>
    <p begin="5171200000t" end="5188800000t" region="Region_00" xml:id="subtitle123"><span style="style_2">a las cinco de la mañana?</span><br/><span style="style_2">-Pues no lo sé,</span></p>
    <p begin="5189600000t" end="5217600000t" region="Region_00" xml:id="subtitle124"><span style="style_2">no tengo ni puta idea. No sé</span><br/><span style="style_2">si baila samba o duerme nerviosa.</span></p>
    <p begin="5247200000t" end="5290400000t" region="Region_00" xml:id="subtitle125"><span style="style_2">-Dime, ¿yo te parezco un mamón</span><br/><span style="style_2">al que se pueda mentir</span></p>
    <p begin="5291200000t" end="5312800000t" region="Region_00" xml:id="subtitle126"><span style="style_2">como si le estuvieran</span><br/><span style="style_2">escupiendo en la cara?</span></p>
    <p begin="5372400000t" end="5388000000t" region="Region_01" xml:id="subtitle127"><span style="style_2">Estoy de coña.</span></p>
    <p begin="5444400000t" end="5466800000t" region="Region_01" xml:id="subtitle128"><span style="style_2">Si yo también me la hubiese zumbado.</span></p>
    <p begin="5477200000t" end="5494400000t" region="Region_01" xml:id="subtitle129"><span style="style_2">De hecho, puede que lo intente.</span></p>
    <p begin="5502800000t" end="5523200000t" region="Region_00" xml:id="subtitle130"><span style="style_2">-Me parece</span><br/><span style="style_2">que te estás equivocando, ¿eh?</span></p>
    <p begin="5526400000t" end="5545200000t" region="Region_01" xml:id="subtitle131"><span style="style_2">Voy muy en serio con ella.</span></p>
    <p begin="5554800000t" end="5567600000t" region="Region_01" xml:id="subtitle132"><span style="style_2">-¿Muy en serio?</span></p>
    <p begin="5587200000t" end="5625200000t" region="Region_00" xml:id="subtitle133"><span style="style_2">Pero ¿en serio de tener un jardín</span><br/><span style="style_2">y dedicarte al bricolaje</span></p>
    <p begin="5626000000t" end="5645600000t" region="Region_00" xml:id="subtitle134"><span style="style_2">los domingos?</span><br/><span style="style_2">-Eso es.</span></p>
    <p begin="5648000000t" end="5663600000t" region="Region_01" xml:id="subtitle135"><span style="style_2">Y llenarlo de niños.</span></p>
    <p begin="5735200000t" end="5747600000t" region="Region_01" xml:id="subtitle136"><span style="style_2">-Siéntate.</span></p>
    <p begin="5823600000t" end="5863600000t" region="Region_00" xml:id="subtitle137"><span style="style_2">Mira, chico, las mujeres</span><br/><span style="style_2">te darán sexo y diversión</span></p>
    <p begin="5864400000t" end="5909600000t" region="Region_00" xml:id="subtitle138"><span style="style_2">porque... están programadas</span><br/><span style="style_2">para doblegarte y que las fecundes.</span></p>
    <p begin="5917200000t" end="5932400000t" region="Region_01" xml:id="subtitle139"><span style="style_2">Después dejarás de existir.</span></p>
    <p begin="5941200000t" end="5961600000t" region="Region_00" xml:id="subtitle140"><span style="style_2">En el parto</span><br/><span style="style_2">te vas a dar cuenta de eso.</span></p>
    <p begin="5973200000t" end="6003200000t" region="Region_00" xml:id="subtitle141"><span style="style_2">-El parto tiene que ser lo más</span><br/><span style="style_2">emocionante en la vida de un padre.</span></p>
    <p begin="6004800000t" end="6042400000t" region="Region_00" xml:id="subtitle142"><span style="style_2">-En el parto</span><br/><span style="style_2">lo que va a salir entre sus piernas</span></p>
    <p begin="6043200000t" end="6070400000t" region="Region_00" xml:id="subtitle143"><span style="style_2">es una cabeza nuclear</span><br/><span style="style_2">que lo va a arrasar todo.</span></p>
    <p begin="6086800000t" end="6116800000t" region="Region_00" xml:id="subtitle144"><span style="style_2">En primer lugar, la maravillosa</span><br/><span style="style_2">cueva en la que tú metías tu picha</span></p>
    <p begin="6117600000t" end="6130400000t" region="Region_01" xml:id="subtitle145"><span style="style_2">ya nunca volverá a su ser.</span></p>
    <p begin="6133200000t" end="6162800000t" region="Region_00" xml:id="subtitle146"><span style="style_2">Y mientras maldice tu nombre</span><br/><span style="style_2">y pide la epidural,</span></p>
    <p begin="6163600000t" end="6180400000t" region="Region_01" xml:id="subtitle147"><span style="style_2">se hará de vientre encima.</span></p>
    <p begin="6206400000t" end="6226000000t" region="Region_00" xml:id="subtitle148"><span style="style_2">¿Sabes lo que te está diciendo</span><br/><span style="style_2">con eso?</span></p>
    <p begin="6235600000t" end="6258000000t" region="Region_00" xml:id="subtitle149"><span style="style_2">Que ya nunca volverá a ser</span><br/><span style="style_2">una mujer sexy.</span></p>
    <p begin="6274800000t" end="6303200000t" region="Region_00" xml:id="subtitle150"><span style="style_2">Y que a partir de ahora</span><br/><span style="style_2">ese cebón se va a convertir</span></p>
    <p begin="6304000000t" end="6322000000t" region="Region_01" xml:id="subtitle151"><span style="style_2">en el centro del universo.</span></p>
    <p begin="6332400000t" end="6344000000t" region="Region_01" xml:id="subtitle152"><span style="style_2">Todas son así.</span></p>
    <p begin="6367200000t" end="6386800000t" region="Region_00" xml:id="subtitle153"><span style="style_2">Te lo digo yo,</span><br/><span style="style_2">que he tenido cinco divorcios.</span></p>
    <p begin="6387600000t" end="6403200000t" region="Region_01" xml:id="subtitle154"><span style="style_2">¿Sabes lo que son cinco divorcios?</span></p>
    <p begin="6431600000t" end="6451200000t" region="Region_01" xml:id="subtitle155"><span style="style_2">Cinco veces que creí en el amor.</span></p>
    <p begin="6492400000t" end="6511600000t" region="Region_00" xml:id="subtitle156"><span style="style_2">-¿Tú qué puta mierda</span><br/><span style="style_2">tienes en la cabeza?</span></p>
    <p begin="6540000000t" end="6551600000t" region="Region_01" xml:id="subtitle157"><span style="style_2">¿Eh?</span></p>
    <p begin="6561600000t" end="6592800000t" region="Region_00" xml:id="subtitle158"><span style="style_2">¿Cómo coño te ha puesto el Profesor</span><br/><span style="style_2">al mando de esto?</span></p>
    <p begin="6607600000t" end="6631600000t" region="Region_00" xml:id="subtitle159"><span style="style_2">-Por mi sensibilidad</span><br/><span style="style_2">para tratar a las personas.</span></p>
    <p begin="6695200000t" end="6724000000t" region="Region_00" xml:id="subtitle160"><span style="style_2">Ahora saca al corderito del grupo</span><br/><span style="style_2">y llévalo a un despacho.</span></p>
    <p begin="6724800000t" end="6736000000t" region="Region_01" xml:id="subtitle161"><span style="style_2">Y no te separes de ella.</span></p>
    <p begin="6736800000t" end="6757600000t" region="Region_00" xml:id="subtitle162"><span style="style_2">Si se va a cambiar la compresa,</span><br/><span style="style_2">tú a su lado.</span></p>
    <p begin="6768400000t" end="6778800000t" region="Region_01" xml:id="subtitle163"><span style="style_2">-Vale.</span></p>
    <p begin="6876800000t" end="6897600000t" region="Region_01" xml:id="subtitle164"><span style="style_2">(MURMURAN A LO LEJOS)</span></p>
    <p begin="6943200000t" end="6971600000t" region="Region_00" xml:id="subtitle165"><span style="style_1">Volvemos en 30 minutos.</span><br/><span style="style_1">30 minutos, Denver.</span></p>
    <p begin="6990800000t" end="7001600000t" region="Region_01" xml:id="subtitle166"><span style="style_4">Permiso.</span></p>
    <p begin="7071200000t" end="7099600000t" region="Region_00" xml:id="subtitle167"><span style="style_2">Eh... Bueno,</span><br/><span style="style_2">yo voy a correr un poquito,</span></p>
    <p begin="7100400000t" end="7122000000t" region="Region_01" xml:id="subtitle168"><span style="style_2">a estirar un poco las piernas.</span></p>
    <p begin="7127200000t" end="7143600000t" region="Region_01" xml:id="subtitle169"><span style="style_2">-Tírale, ¿será por campo?</span></p>
    <p begin="7144400000t" end="7175200000t" region="Region_00" xml:id="subtitle170"><span style="style_1">Aprender un idioma</span><br/><span style="style_1">es mucho más fácil de lo que parece,</span></p>
    <p begin="7176000000t" end="7210000000t" region="Region_00" xml:id="subtitle171"><span style="style_1">pero tienes que dejar</span><br/><span style="style_1">la semántica a un lado.</span></p>
    <p begin="8031200000t" end="8040000000t" region="Region_01" xml:id="subtitle172"><span style="style_2">Vamos.</span></p>
    <p begin="8052800000t" end="8091600000t" region="Region_00" xml:id="subtitle173"><span style="style_2">Tranquila, voy a llevarte a otro</span><br/><span style="style_2">lugar para que descanses, nada más.</span></p>
    <p begin="8092400000t" end="8100800000t" region="Region_01" xml:id="subtitle174"><span style="style_2">¿Vale?</span></p>
    <p begin="8170000000t" end="8198800000t" region="Region_00" xml:id="subtitle175"><span style="style_2">-Raquel, todas las comunicaciones</span><br/><span style="style_2">que hagamos desde dentro</span></p>
    <p begin="8199600000t" end="8231200000t" region="Region_00" xml:id="subtitle176"><span style="style_2">deben hacerse desde este teléfono</span><br/><span style="style_2">o la radio, nada de móviles, ¿vale?</span></p>
    <p begin="8232000000t" end="8266800000t" region="Region_00" xml:id="subtitle177"><span style="style_2">-Ponme con el interior,</span><br/><span style="style_2">quiero hablar con los atracadores.</span></p>
    <p begin="8276000000t" end="8309600000t" region="Region_00" xml:id="subtitle178"><span style="style_2">Ángel, ¿qué coño hace</span><br/><span style="style_2">el coronel Prieto aquí?</span></p>
    <p begin="8322000000t" end="8340400000t" region="Region_01" xml:id="subtitle179"><span style="style_2">-No lo sé. No lo sé.</span></p>
    <p begin="8358000000t" end="8379600000t" region="Region_00" xml:id="subtitle180"><span style="style_2">-Hay unos individuos</span><br/><span style="style_2">pegando tiros en el sitio</span></p>
    <p begin="8380400000t" end="8394000000t" region="Region_01" xml:id="subtitle181"><span style="style_2">donde se hace el dinero,</span></p>
    <p begin="8404400000t" end="8426400000t" region="Region_00" xml:id="subtitle182"><span style="style_2">¿no cree que Inteligencia</span><br/><span style="style_2">debe estar al tanto?</span></p>
    <p begin="8430000000t" end="8456000000t" region="Region_00" xml:id="subtitle183"><span style="style_2">-Llamando a la Fábrica</span><br/><span style="style_2">de Moneda y Timbre, Raquel.</span></p>
    <p begin="8461200000t" end="8482800000t" region="Region_00" xml:id="subtitle184"><span style="style_2">(AGENTE) Jefe, llamada</span><br/><span style="style_2">al interior en curso.</span></p>
    <p begin="8491200000t" end="8517200000t" region="Region_02" xml:id="subtitle185"><span style="style_2">(Tono de llamada)</span></p>
    <p begin="8519200000t" end="8542000000t" region="Region_02" xml:id="subtitle186"><span style="style_2">(Timbre)</span></p>
    <p begin="8619200000t" end="8639200000t" region="Region_02" xml:id="subtitle187"><span style="style_2">(Tono de llamada)</span></p>
    <p begin="8673600000t" end="8697200000t" region="Region_02" xml:id="subtitle188"><span style="style_2">(Timbre)</span></p>
    <p begin="8698000000t" end="8715600000t" region="Region_02" xml:id="subtitle189"><span style="style_2">(Teléfono)</span></p>
    <p begin="8750800000t" end="8760000000t" region="Region_01" xml:id="subtitle190"><span style="style_2">(RAQUEL) "Hola".</span></p>
    <p begin="8762000000t" end="8783600000t" region="Region_00" xml:id="subtitle191"><span style="style_1">Buenas tardes.</span><br/><span style="style_2">"Soy Raquel Murillo,"</span></p>
    <p begin="8784400000t" end="8813200000t" region="Region_00" xml:id="subtitle192"><span style="style_2">inspectora al mando de la gestión</span><br/><span style="style_2">del atraco. ¿Con quién hablo?</span></p>
    <p begin="8816800000t" end="8844000000t" region="Region_00" xml:id="subtitle193"><span style="style_1">(DISTORSIONADO) "Con el atracador</span><br/><span style="style_1">al mando del asalto".</span></p>
    <p begin="8844800000t" end="8870800000t" region="Region_00" xml:id="subtitle194"><span style="style_2">-Tiene modificador de voz.</span><br/><span style="style_2">-Limpiando la señal.</span></p>
    <p begin="8871600000t" end="8889600000t" region="Region_01" xml:id="subtitle195"><span style="style_1">¿Cómo se encuentran sus compañeros?</span></p>
    <p begin="8917200000t" end="8947200000t" region="Region_00" xml:id="subtitle196"><span style="style_2">(RAQUEL) Por el momento no tenemos</span><br/><span style="style_2">que lamentar ninguna baja.</span></p>
    <p begin="8954000000t" end="8961200000t" region="Region_01" xml:id="subtitle197"><span style="style_1">"Me alegro".</span></p>
    <p begin="8985600000t" end="9001600000t" region="Region_01" xml:id="subtitle198"><span style="style_1">"De verdad, sinceramente".</span></p>
    <p begin="9035600000t" end="9058800000t" region="Region_01" xml:id="subtitle199"><span style="style_2">Perdone, le oigo un poco raro.</span></p>
    <p begin="9061600000t" end="9088800000t" region="Region_00" xml:id="subtitle200"><span style="style_1">Sí, le pido disculpas</span><br/><span style="style_1">por esta odiosa voz metálica,</span></p>
    <p begin="9089600000t" end="9126000000t" region="Region_00" xml:id="subtitle201"><span style="style_1">"pero comprenderá</span><br/><span style="style_1">que debo salvaguardar mi identidad,"</span></p>
    <p begin="9126800000t" end="9154400000t" region="Region_00" xml:id="subtitle202"><span style="style_1">por si algún día nos ponen</span><br/><span style="style_1">un helicóptero y nos vamos a Brasil.</span></p>
    <p begin="9157600000t" end="9181600000t" region="Region_00" xml:id="subtitle203"><span style="style_2">¿Eso es lo que quiere,</span><br/><span style="style_2">un helicóptero?</span></p>
    <p begin="9182400000t" end="9204400000t" region="Region_00" xml:id="subtitle204"><span style="style_1">"De momento lo que quiero</span><br/><span style="style_1">es negociar con alguien"</span></p>
    <p begin="9205200000t" end="9233600000t" region="Region_00" xml:id="subtitle205"><span style="style_1">que no me esté dando largas,</span><br/><span style="style_1">que no tenga que preguntar</span></p>
    <p begin="9234400000t" end="9272400000t" region="Region_00" xml:id="subtitle206"><span style="style_1">a un superior, a Inteligencia o</span><br/><span style="style_1">su mamá para decirme un sí o un no.</span></p>
    <p begin="9273200000t" end="9295600000t" region="Region_00" xml:id="subtitle207"><span style="style_2">Entonces debería hablar</span><br/><span style="style_2">con el presidente,</span></p>
    <p begin="9296400000t" end="9319600000t" region="Region_00" xml:id="subtitle208"><span style="style_2">pero puesto que está ocupado</span><br/><span style="style_2">dirigiendo el país,</span></p>
    <p begin="9320400000t" end="9332800000t" region="Region_01" xml:id="subtitle209"><span style="style_2">intentaré sustituirle,</span></p>
    <p begin="9333600000t" end="9347600000t" region="Region_01" xml:id="subtitle210"><span style="style_2">"si no le parece mal".</span></p>
    <p begin="9365200000t" end="9394400000t" region="Region_00" xml:id="subtitle211"><span style="style_2">¿Alguna pregunta más?</span><br/><span style="style_1">"Sí".</span></p>
    <p begin="9417200000t" end="9427600000t" region="Region_01" xml:id="subtitle212"><span style="style_1">¿Qué lleva puesto?</span></p>
    <p begin="9465200000t" end="9490400000t" region="Region_00" xml:id="subtitle213"><span style="style_2">¿Cómo?</span><br/><span style="style_1">"¿Cómo va vestida?</span></p>
    <p begin="9501200000t" end="9515200000t" region="Region_01" xml:id="subtitle214"><span style="style_1">¿No le parece que nuestra ropa"</span></p>
    <p begin="9516000000t" end="9533600000t" region="Region_01" xml:id="subtitle215"><span style="style_1">habla mucho de nuestra personalidad?</span></p>
    <p begin="9558000000t" end="9590800000t" region="Region_00" xml:id="subtitle216"><span style="style_2">Mire, no tengo inconveniente</span><br/><span style="style_2">en contestar a su pregunta,</span></p>
    <p begin="9591600000t" end="9611200000t" region="Region_01" xml:id="subtitle217"><span style="style_2">pero creo que debería informarle</span></p>
    <p begin="9612000000t" end="9640800000t" region="Region_00" xml:id="subtitle218"><span style="style_2">"que esta conversación está siendo</span><br/><span style="style_2">escuchada por varios miembros</span></p>
    <p begin="9641600000t" end="9659600000t" region="Region_01" xml:id="subtitle219"><span style="style_2">de la UDEF, de la UIT, el CNI"</span></p>
    <p begin="9660400000t" end="9702000000t" region="Region_00" xml:id="subtitle220"><span style="style_2">y su gabinete de enlace, el jefe</span><br/><span style="style_2">de los GEO y varios oficiales más.</span></p>
    <p begin="9712800000t" end="9748000000t" region="Region_00" xml:id="subtitle221"><span style="style_1">En ese caso, creo que lo cortés</span><br/><span style="style_1">es saludarles y presentarnos.</span></p>
    <p begin="9750800000t" end="9769200000t" region="Region_01" xml:id="subtitle222"><span style="style_1">Señores, un placer.</span></p>
    <p begin="9770000000t" end="9796400000t" region="Region_00" xml:id="subtitle223"><span style="style_1">"Disculpen que no les dé mi nombre,</span><br/><span style="style_1">pero pueden llamarme"</span></p>
    <p begin="9797200000t" end="9828000000t" region="Region_00" xml:id="subtitle224"><span style="style_1">Profesor,</span><br/><span style="style_1">es como me llama todo el mundo.</span></p>
    <p begin="9875600000t" end="9906400000t" region="Region_00" xml:id="subtitle225"><span style="style_2">Hola.</span><br/><span style="style_2">Soy Ángel, subinspector de Policía.</span></p>
    <p begin="9907200000t" end="9928000000t" region="Region_00" xml:id="subtitle226"><span style="style_1">Encantado, Ángel.</span><br/><span style="style_1">¿Qué tal, cómo está?</span></p>
    <p begin="9942000000t" end="9962000000t" region="Region_00" xml:id="subtitle227"><span style="style_2">-"Hola, buenos días".</span><br/><span style="style_1">Buenos días.</span></p>
    <p begin="9962800000t" end="9976800000t" region="Region_01" xml:id="subtitle228"><span style="style_2">-Hola, ¿qué tal?</span></p>
    <p begin="9977600000t" end="10007200000t" region="Region_00" xml:id="subtitle229"><span style="style_1">Muy bien. Yo muy bien,</span><br/><span style="style_1">muchas gracias. ¿Y usted?</span></p>
    <p begin="10008400000t" end="10062800000t" region="Region_00" xml:id="subtitle230"><span style="style_2">Y después de este emotivo momento,</span><br/><span style="style_2">dígame,</span></p>
    <p begin="10070400000t" end="10098400000t" region="Region_00" xml:id="subtitle231"><span style="style_2">¿qué es lo que quiere?</span><br/><span style="style_1">Tiempo...</span></p>
    <p begin="10115200000t" end="10133600000t" region="Region_01" xml:id="subtitle232"><span style="style_1">"para intentar aclarar las cosas.</span></p>
    <p begin="10134400000t" end="10146800000t" region="Region_01" xml:id="subtitle233"><span style="style_1">Escúcheme, inspectora.</span></p>
    <p begin="10147600000t" end="10176000000t" region="Region_00" xml:id="subtitle234"><span style="style_1">No hemos podido salir por cuestión</span><br/><span style="style_1">de segundos, pero estamos"</span></p>
    <p begin="10176800000t" end="10200800000t" region="Region_00" xml:id="subtitle235"><span style="style_1">perfectamente preparados</span><br/><span style="style_1">para defendernos,</span></p>
    <p begin="10201600000t" end="10232400000t" region="Region_00" xml:id="subtitle236"><span style="style_1">"así que intente evitar</span><br/><span style="style_1">cualquier tipo de intervención".</span></p>
    <p begin="10255600000t" end="10281600000t" region="Region_00" xml:id="subtitle237"><span style="style_1">¿Puedo confiar en usted?</span><br/><span style="style_2">Por supuesto.</span></p>
    <p begin="10295600000t" end="10332400000t" region="Region_00" xml:id="subtitle238"><span style="style_2">Pero para que yo confíe en usted</span><br/><span style="style_2">debe tener un gesto conmigo.</span></p>
    <p begin="10348000000t" end="10372800000t" region="Region_00" xml:id="subtitle239"><span style="style_2">Libere a esos chavales del colegio,</span><br/><span style="style_2">son menores.</span></p>
    <p begin="10373600000t" end="10387200000t" region="Region_01" xml:id="subtitle240"><span style="style_2">Todo irá mucho mejor así.</span></p>
    <p begin="10394800000t" end="10429600000t" region="Region_00" xml:id="subtitle241"><span style="style_1">Pero, inspectora, usted todavía</span><br/><span style="style_1">no me ha respondido cómo va vestida.</span></p>
    <p begin="10462800000t" end="10506000000t" region="Region_00" xml:id="subtitle242"><span style="style_2">Con un traje de chaqueta gris,</span><br/><span style="style_2">una camisa azul,</span></p>
    <p begin="10514400000t" end="10536400000t" region="Region_01" xml:id="subtitle243"><span style="style_2">"unas botas de tacón negras y...".</span></p>
    <p begin="10538400000t" end="10565600000t" region="Region_00" xml:id="subtitle244"><span style="style_2">Bueno, creo que por el momento</span><br/><span style="style_2">es suficiente, ¿no?</span></p>
    <p begin="10566400000t" end="10591600000t" region="Region_00" xml:id="subtitle245"><span style="style_1">"Así puedo imaginármela</span><br/><span style="style_1">muchísimo mejor".</span></p>
    <p begin="10592400000t" end="10619200000t" region="Region_00" xml:id="subtitle246"><span style="style_1">Pero, inspectora,</span><br/><span style="style_1">un traje de chaqueta para mujeres</span></p>
    <p begin="10620000000t" end="10636800000t" region="Region_01" xml:id="subtitle247"><span style="style_1">puede ser de falda o de pantalón,</span></p>
    <p begin="10637600000t" end="10668800000t" region="Region_00" xml:id="subtitle248"><span style="style_1">"así que le pediría</span><br/><span style="style_1">que la próxima vez fuera usted"</span></p>
    <p begin="10669600000t" end="10685600000t" region="Region_01" xml:id="subtitle249"><span style="style_1">un poquito más concreta.</span></p>
    <p begin="10712800000t" end="10728000000t" region="Region_01" xml:id="subtitle250"><span style="style_1">Déjeme pensar en su petición.</span></p>
    <p begin="10774800000t" end="10796400000t" region="Region_00" xml:id="subtitle251"><span style="style_2">A ver, ¿qué clase de zumbado</span><br/><span style="style_2">con 60 rehenes</span></p>
    <p begin="10797200000t" end="10815200000t" region="Region_01" xml:id="subtitle252"><span style="style_2">se pone a vacilar a su negociadora?</span></p>
    <p begin="10844400000t" end="10887200000t" region="Region_00" xml:id="subtitle253"><span style="style_2">-No, está demasiado tranquilo</span><br/><span style="style_2">para ser un zumbado.</span></p>
    <p begin="10896800000t" end="10929200000t" region="Region_00" xml:id="subtitle254"><span style="style_2">¡Quiero helicópteros volando</span><br/><span style="style_2">todo el día encima de su cabeza!</span></p>
    <p begin="10930000000t" end="10962400000t" region="Region_00" xml:id="subtitle255"><span style="style_2">Nada de comida, agua o cualquier</span><br/><span style="style_2">otro tipo de petición. ¿Está claro?</span></p>
    <p begin="10963200000t" end="10978400000t" region="Region_01" xml:id="subtitle256"><span style="style_2">-Disculpe, inspectora.</span></p>
    <p begin="10984000000t" end="11016800000t" region="Region_00" xml:id="subtitle257"><span style="style_2">Pero porque los deje sin postre</span><br/><span style="style_2">no creo que vayan a entregarse.</span></p>
    <p begin="11026800000t" end="11050400000t" region="Region_00" xml:id="subtitle258"><span style="style_2">Tanto el jefe de los GEO</span><br/><span style="style_2">aquí presente como yo</span></p>
    <p begin="11051200000t" end="11079200000t" region="Region_00" xml:id="subtitle259"><span style="style_2">creemos que es necesaria</span><br/><span style="style_2">una intervención inmediata.</span></p>
    <p begin="11092000000t" end="11119200000t" region="Region_00" xml:id="subtitle260"><span style="style_2">-Si hubieran querido una carnicería</span><br/><span style="style_2">como la de la ópera de Moscú,</span></p>
    <p begin="11120000000t" end="11140800000t" region="Region_00" xml:id="subtitle261"><span style="style_2">le hubieran llamado directamente</span><br/><span style="style_2">a usted,</span></p>
    <p begin="11145200000t" end="11175600000t" region="Region_00" xml:id="subtitle262"><span style="style_2">pero desgraciadamente</span><br/><span style="style_2">solo está aquí para opinar y joder.</span></p>
    <p begin="11176400000t" end="11210400000t" region="Region_00" xml:id="subtitle263"><span style="style_2">-Por favor, inspectora,</span><br/><span style="style_2">no se ponga a la defensiva.</span></p>
    <p begin="11211200000t" end="11225200000t" region="Region_01" xml:id="subtitle264"><span style="style_2">Estoy aquí para ayudarla.</span></p>
    <p begin="11226000000t" end="11249200000t" region="Region_00" xml:id="subtitle265"><span style="style_2">Sé que no está pasando</span><br/><span style="style_2">por su mejor momento.</span></p>
    <p begin="11250000000t" end="11274400000t" region="Region_00" xml:id="subtitle266"><span style="style_2">-Si se refiere usted</span><br/><span style="style_2">a la menstruación, no es el caso,</span></p>
    <p begin="11275200000t" end="11291200000t" region="Region_01" xml:id="subtitle267"><span style="style_2">pero gracias por preguntar.</span></p>
    <p begin="11294000000t" end="11317200000t" region="Region_00" xml:id="subtitle268"><span style="style_2">-Me refiero</span><br/><span style="style_2">a la denuncia por malos tratos</span></p>
    <p begin="11318000000t" end="11336400000t" region="Region_01" xml:id="subtitle269"><span style="style_2">que ha interpuesto a su exmarido.</span></p>
    <p begin="11363600000t" end="11381600000t" region="Region_02" xml:id="subtitle270"><span style="style_2">(Murmuraciones)</span></p>
    <p begin="11412400000t" end="11438800000t" region="Region_00" xml:id="subtitle271"><span style="style_2">-Vaya, qué bien hace</span><br/><span style="style_2">los deberes Inteligencia.</span></p>
    <p begin="11444800000t" end="11475600000t" region="Region_00" xml:id="subtitle272"><span style="style_2">No queda una sola alcoba</span><br/><span style="style_2">donde no metan sus narices, ¿eh?</span></p>
    <p begin="11476400000t" end="11496000000t" region="Region_01" xml:id="subtitle273"><span style="style_2">-Asuntos Internos también lo hace.</span></p>
    <p begin="11499600000t" end="11528800000t" region="Region_00" xml:id="subtitle274"><span style="style_2">Cree que usted</span><br/><span style="style_2">ha interpuesto una denuncia falsa.</span></p>
    <p begin="11544000000t" end="11557600000t" region="Region_01" xml:id="subtitle275"><span style="style_2">Su exmarido ha rehecho su vida</span></p>
    <p begin="11558400000t" end="11586000000t" region="Region_00" xml:id="subtitle276"><span style="style_2">tanto personal como profesionalmente</span><br/><span style="style_2">dentro del Cuerpo,</span></p>
    <p begin="11595200000t" end="11619200000t" region="Region_00" xml:id="subtitle277"><span style="style_2">pero hay mujeres</span><br/><span style="style_2">que no siempre pasan página.</span></p>
    <p begin="11630400000t" end="11659600000t" region="Region_00" xml:id="subtitle278"><span style="style_2">-Mire,</span><br/><span style="style_2">no sé si es usted un miserable</span></p>
    <p begin="11660400000t" end="11687200000t" region="Region_01" xml:id="subtitle279"><span style="style_2">o... simplemente un necio.</span></p>
    <p begin="11700400000t" end="11730400000t" region="Region_00" xml:id="subtitle280"><span style="style_2">Por si a alguien</span><br/><span style="style_2">le interesa mi vida personal,</span></p>
    <p begin="11731200000t" end="11760000000t" region="Region_00" xml:id="subtitle281"><span style="style_2">he de decir que es</span><br/><span style="style_2">un asunto ampliamente superado</span></p>
    <p begin="11760800000t" end="11774400000t" region="Region_01" xml:id="subtitle282"><span style="style_2">y está en manos de un juez.</span></p>
    <p begin="11813600000t" end="11845200000t" region="Region_00" xml:id="subtitle283"><span style="style_2">Gracias por su sensibilidad,</span><br/><span style="style_2">coronel.</span></p>
    <p begin="11865600000t" end="11881600000t" region="Region_02" xml:id="subtitle284"><span style="style_2">(Alarma)</span></p>
    <p begin="11896400000t" end="11922000000t" region="Region_00" xml:id="subtitle285"><span style="style_2">-Está entrando una llamada</span><br/><span style="style_2">en el repetidor del museo.</span></p>
    <p begin="11927200000t" end="11943600000t" region="Region_00" xml:id="subtitle286"><span style="style_2">-Graba.</span><br/><span style="style_2">-Sí, estamos grabando.</span></p>
    <p begin="11944400000t" end="11971600000t" region="Region_00" xml:id="subtitle287"><span style="style_2">-Sube el volumen.</span><br/><span style="style_2">¡Silencio, por favor!</span></p>
    <p begin="11983200000t" end="12005200000t" region="Region_00" xml:id="subtitle288"><span style="style_2">-"¿Cómo que no vienes a cenar?</span><br/><span style="style_2">-Estoy trabajando.</span></p>
    <p begin="12006000000t" end="12030400000t" region="Region_00" xml:id="subtitle289"><span style="style_2">Discúlpame</span><br/><span style="style_2">delante de tu madre y tu hermana.</span></p>
    <p begin="12031200000t" end="12042800000t" region="Region_01" xml:id="subtitle290"><span style="style_2">-¿Cómo que no vienes?".</span></p>
    <p begin="12043600000t" end="12078400000t" region="Region_00" xml:id="subtitle291"><span style="style_2">-Se ha liado una buena. Discúlpame</span><br/><span style="style_2">delante de ellas, pero créetelo.</span></p>
    <p begin="12079200000t" end="12103600000t" region="Region_00" xml:id="subtitle292"><span style="style_2">No me chilles, bonita, por favor.</span><br/><span style="style_2">-(A LO LEJOS) ¿Qué pasa?</span></p>
    <p begin="12104400000t" end="12122400000t" region="Region_01" xml:id="subtitle293"><span style="style_2">-Bueno, te dejo, ¿vale, cosita?</span></p>
    <p begin="12123200000t" end="12148400000t" region="Region_00" xml:id="subtitle294"><span style="style_2">Venga, que te quiero un montón.</span><br/><span style="style_2">Adiós, adiós.</span></p>
    <p begin="12171600000t" end="12181200000t" region="Region_01" xml:id="subtitle295"><span style="style_2">-Corta.</span></p>
    <p begin="12191200000t" end="12203200000t" region="Region_01" xml:id="subtitle296"><span style="style_2">Es Almansa.</span></p>
    <p begin="12246800000t" end="12276400000t" region="Region_02" xml:id="subtitle297"><span style="style_2">(Helicópteros)</span></p>
    <p begin="12517600000t" end="12532400000t" region="Region_01" xml:id="subtitle298"><span style="style_2">¿Por qué me has traído aquí?</span></p>
    <p begin="12592000000t" end="12605200000t" region="Region_01" xml:id="subtitle299"><span style="style_2">-No, tranquila,</span></p>
    <p begin="12614400000t" end="12631200000t" region="Region_00" xml:id="subtitle300"><span style="style_2">me han traído aquí</span><br/><span style="style_2">para que te proteja.</span></p>
    <p begin="12645600000t" end="12654000000t" region="Region_01" xml:id="subtitle301"><span style="style_2">Solo eso.</span></p>
    <p begin="12663200000t" end="12685200000t" region="Region_00" xml:id="subtitle302"><span style="style_2">Cuando te desnudes, por favor,</span><br/><span style="style_2">ponte el mono rojo.</span></p>
    <p begin="12693200000t" end="12703600000t" region="Region_01" xml:id="subtitle303"><span style="style_2">-Vale.</span></p>
    <p begin="12986800000t" end="13006800000t" region="Region_02" xml:id="subtitle304"><span style="style_2">(Helicóptero)</span></p>
    <p begin="13036000000t" end="13049200000t" region="Region_01" xml:id="subtitle305"><span style="style_2">(RÍO) ¿Y si no sale bien?</span></p>
    <p begin="13052400000t" end="13077200000t" region="Region_00" xml:id="subtitle306"><span style="style_2">¿Qué va a pasar si no sale bien?</span><br/><span style="style_2">(NAIROBI) A ver, cachorrito,</span></p>
    <p begin="13078000000t" end="13102000000t" region="Region_00" xml:id="subtitle307"><span style="style_2">lo mismo de siempre:</span><br/><span style="style_2">vuelta al trullo,</span></p>
    <p begin="13103200000t" end="13142800000t" region="Region_00" xml:id="subtitle308"><span style="style_2">al cigarrito en el patio, a</span><br/><span style="style_2">los cuatro langostinos por Navidad,</span></p>
    <p begin="13146400000t" end="13171200000t" region="Region_00" xml:id="subtitle309"><span style="style_2">los vis a vis, a veces.</span><br/><span style="style_2">-Un lujo, vamos.</span></p>
    <p begin="13172000000t" end="13205200000t" region="Region_00" xml:id="subtitle310"><span style="style_4">Lo jodido es si sale bien. ¿Qué coño</span><br/><span style="style_4">vamos a hacer con tanta pasta?</span></p>
    <p begin="13206000000t" end="13226800000t" region="Region_01" xml:id="subtitle311"><span style="style_2">-Yo me voy a pillar un Maserati,</span></p>
    <p begin="13227600000t" end="13254400000t" region="Region_00" xml:id="subtitle312"><span style="style_2">color azul cielo despejado,</span><br/><span style="style_2">¿eh? (RÍE)</span></p>
    <p begin="13259600000t" end="13280400000t" region="Region_00" xml:id="subtitle313"><span style="style_2">-Di que sí.</span><br/><span style="style_2">-Y un gimnasio de artes marciales.</span></p>
    <p begin="13281200000t" end="13296000000t" region="Region_00" xml:id="subtitle314"><span style="style_2">(VARIOS RÍEN)</span><br/><span style="style_2">(NAIROBI) Lo veo.</span></p>
    <p begin="13296800000t" end="13316400000t" region="Region_00" xml:id="subtitle315"><span style="style_2">(DENVER) Y un garitazo,</span><br/><span style="style_2">de tres pisos, ¿eh?</span></p>
    <p begin="13317200000t" end="13338000000t" region="Region_00" xml:id="subtitle316"><span style="style_2">Con unos altavoces</span><br/><span style="style_2">que te sangren los oídos.</span></p>
    <p begin="13338800000t" end="13371200000t" region="Region_00" xml:id="subtitle317"><span style="style_2">Pum, pum, pam, que pim pam pum.</span><br/><span style="style_2">-Di que sí.</span></p>
    <p begin="13372000000t" end="13402000000t" region="Region_00" xml:id="subtitle318"><span style="style_2">-A este con tres millones le sobra.</span><br/><span style="style_2">-Y para ti unos pulmones,</span></p>
    <p begin="13402800000t" end="13420000000t" region="Region_00" xml:id="subtitle319"><span style="style_2">que los tienes hechos mierda</span><br/><span style="style_2">de la mina.</span></p>
    <p begin="13420800000t" end="13445200000t" region="Region_00" xml:id="subtitle320"><span style="style_2">-¿En eso te lo vas a gastar,</span><br/><span style="style_2">en unos pulmones?</span></p>
    <p begin="13446000000t" end="13460800000t" region="Region_01" xml:id="subtitle321"><span style="style_2">Anda que ya te vale.</span></p>
    <p begin="13468400000t" end="13494400000t" region="Region_00" xml:id="subtitle322"><span style="style_4">¿Y se puede saber de dónde coño</span><br/><span style="style_4">vas a sacar unos pulmones?</span></p>
    <p begin="13495200000t" end="13523200000t" region="Region_00" xml:id="subtitle323"><span style="style_2">-¿No hay gente que vende riñones?</span><br/><span style="style_2">Habrá gente que venda pulmones,</span></p>
    <p begin="13524000000t" end="13535200000t" region="Region_00" xml:id="subtitle324"><span style="style_2">digo yo.</span><br/><span style="style_2">(RÍEN)</span></p>
    <p begin="13536000000t" end="13566800000t" region="Region_00" xml:id="subtitle325"><span style="style_2">-Pues yo... en una bodega</span><br/><span style="style_2">en la Provenza.</span></p>
    <p begin="13567600000t" end="13600000000t" region="Region_00" xml:id="subtitle326"><span style="style_2">100 hectáreas de viñedo</span><br/><span style="style_2">para cultivar mi propio vino,</span></p>
    <p begin="13617600000t" end="13634000000t" region="Region_01" xml:id="subtitle327"><span style="style_2">barricas de roble...</span></p>
    <p begin="13634800000t" end="13657600000t" region="Region_00" xml:id="subtitle328"><span style="style_2">-Pero, tío, puedes ir al</span><br/><span style="style_2">supermercado que te dé la gana</span></p>
    <p begin="13658400000t" end="13678000000t" region="Region_00" xml:id="subtitle329"><span style="style_2">y comprar la botella</span><br/><span style="style_2">que te salga de los huevos,</span></p>
    <p begin="13678800000t" end="13710800000t" region="Region_00" xml:id="subtitle330"><span style="style_2">¿para qué vas a montarte una bodega?</span><br/><span style="style_2">-Por el arte.</span></p>
    <p begin="13711600000t" end="13724800000t" region="Region_01" xml:id="subtitle331"><span style="style_2">(RÍEN)</span></p>
    <p begin="13728000000t" end="13750000000t" region="Region_01" xml:id="subtitle332"><span style="style_4">Pues yo quiero una isla.</span></p>
    <p begin="13761600000t" end="13785200000t" region="Region_00" xml:id="subtitle333"><span style="style_2">-Y yo otra.</span><br/><span style="style_2">-Pues ponte tres.</span></p>
    <p begin="13788800000t" end="13814000000t" region="Region_00" xml:id="subtitle334"><span style="style_4">Tres son multitud ya.</span><br/><span style="style_2">(NAIROBI) Un archipiélago.</span></p>
    <p begin="13817600000t" end="13856000000t" region="Region_00" xml:id="subtitle335"><span style="style_2">(RÍO) No, pues yo quiero una islita</span><br/><span style="style_2">que tenga una casa enorme,</span></p>
    <p begin="13856800000t" end="13879200000t" region="Region_00" xml:id="subtitle336"><span style="style_2">¿sabes?, con un balconcito</span><br/><span style="style_2">que esté pegado al mar.</span></p>
    <p begin="13885600000t" end="13915200000t" region="Region_00" xml:id="subtitle337"><span style="style_2">Que me levante de la cama</span><br/><span style="style_2">y de cabeza al agua.</span></p>
    <p begin="13916400000t" end="13938400000t" region="Region_01" xml:id="subtitle338"><span style="style_2">-Pues por fin alguien inteligente.</span></p>
    <p begin="13939200000t" end="13962800000t" region="Region_00" xml:id="subtitle339"><span style="style_2">-Yo creo que algunos primero</span><br/><span style="style_2">tenemos que arreglar</span></p>
    <p begin="13963600000t" end="14003600000t" region="Region_00" xml:id="subtitle340"><span style="style_2">algún asuntillo que otro, ¿no?</span><br/><span style="style_2">Vamos, por lo menos yo sí.</span></p>
    <p begin="14009200000t" end="14040000000t" region="Region_00" xml:id="subtitle341"><span style="style_2">Y mira, luego con lo que me sobre</span><br/><span style="style_2">pues para un avión.</span></p>
    <p begin="14047200000t" end="14073600000t" region="Region_00" xml:id="subtitle342"><span style="style_2">Para conducirlo yo.</span><br/><span style="style_4">Pudiendo tener un piloto buenorro.</span></p>
    <p begin="14074400000t" end="14097200000t" region="Region_00" xml:id="subtitle343"><span style="style_2">Que no, que no lo pillas,</span><br/><span style="style_2">lo que mola es vacilar</span></p>
    <p begin="14098000000t" end="14121600000t" region="Region_00" xml:id="subtitle344"><span style="style_2">al de la torre de control</span><br/><span style="style_2">y decirle: "Mira, dame pista</span></p>
    <p begin="14122400000t" end="14134800000t" region="Region_01" xml:id="subtitle345"><span style="style_2">para la más artista".</span></p>
    <p begin="14135600000t" end="14146800000t" region="Region_01" xml:id="subtitle346"><span style="style_2">(RÍEN)</span></p>
    <p begin="14150800000t" end="14184400000t" region="Region_00" xml:id="subtitle347"><span style="style_1">Si compramos todas esas cosas</span><br/><span style="style_1">digamos por un precio...</span></p>
    <p begin="14185200000t" end="14215200000t" region="Region_00" xml:id="subtitle348"><span style="style_1">No sé, un precio caro,</span><br/><span style="style_1">un precio muy caro.</span></p>
    <p begin="14225600000t" end="14262400000t" region="Region_00" xml:id="subtitle349"><span style="style_1">Aun así nos seguiría sobrando</span><br/><span style="style_1">muchísimo dinero.</span></p>
    <p begin="14265600000t" end="14283200000t" region="Region_01" xml:id="subtitle350"><span style="style_1">Si vamos a robar a lo grande,</span></p>
    <p begin="14305600000t" end="14317600000t" region="Region_01" xml:id="subtitle351"><span style="style_1">soñad a lo grande.</span></p>
    <p begin="14318400000t" end="14344400000t" region="Region_00" xml:id="subtitle352"><span style="style_2">-Pues yo me grabaría un disco,</span><br/><span style="style_2">de corridos.</span></p>
    <p begin="14350000000t" end="14372400000t" region="Region_01" xml:id="subtitle353"><span style="style_2">Así, y en la portada todo mi careto.</span></p>
    <p begin="14376400000t" end="14385200000t" region="Region_01" xml:id="subtitle354"><span style="style_2">(RÍEN)</span></p>
    <p begin="14386000000t" end="14418000000t" region="Region_00" xml:id="subtitle355"><span style="style_2">-Vas a ser como el Bertín Osborne,</span><br/><span style="style_2">pero con 30 kilos más.</span></p>
    <p begin="14418800000t" end="14432800000t" region="Region_00" xml:id="subtitle356"><span style="style_2">-Uy, uy.</span><br/><span style="style_2">-¿Qué estás hablando?</span></p>
    <p begin="14433600000t" end="14454400000t" region="Region_00" xml:id="subtitle357"><span style="style_2">Bertín canta rancheras,</span><br/><span style="style_2">yo te he dicho corridos,</span></p>
    <p begin="14455200000t" end="14469200000t" region="Region_01" xml:id="subtitle358"><span style="style_2">que es algo muy diferente.</span></p>
    <p begin="14470000000t" end="14491600000t" region="Region_00" xml:id="subtitle359"><span style="style_2">-Enséñale lo que es un corrido,</span><br/><span style="style_2">papá. Enséñaselo.</span></p>
    <p begin="14492400000t" end="14505600000t" region="Region_00" xml:id="subtitle360"><span style="style_2">-Que no, hombre...</span><br/><span style="style_2">-Venga.</span></p>
    <p begin="14506400000t" end="14518400000t" region="Region_00" xml:id="subtitle361"><span style="style_2">-Venga, venga.</span><br/><span style="style_4">Venga, va.</span></p>
    <p begin="14519200000t" end="14544400000t" region="Region_00" xml:id="subtitle362"><span style="style_2">-Pero si te encanta.</span><br/><span style="style_2">-¿Cómo me voy a poner yo a cantar?</span></p>
    <p begin="14545200000t" end="14576800000t" region="Region_00" xml:id="subtitle363"><span style="style_2">(TODOS) Canta, canta.</span><br/><span style="style_4">Se ha levantado y todo.</span></p>
    <p begin="14584800000t" end="14628000000t" region="Region_00" xml:id="subtitle364"><span style="style_2">-# Por la falta de tus labios</span><br/><span style="style_2">lloré por primera vez</span></p>
    <p begin="14631600000t" end="14698000000t" region="Region_00" xml:id="subtitle365"><span style="style_2"># y maldije conocerte</span><br/><span style="style_2">por no dejar de quererte.</span></p>
    <p begin="14700000000t" end="14726400000t" region="Region_01" xml:id="subtitle366"><span style="style_2"># Yo mi esposa quise hacerte,</span></p>
    <p begin="14730400000t" end="14758800000t" region="Region_01" xml:id="subtitle367"><span style="style_2"># sin amor busqué la suerte,</span></p>
    <p begin="14761200000t" end="14792000000t" region="Region_01" xml:id="subtitle368"><span style="style_2"># fui tirando de pistola</span></p>
    <p begin="14794000000t" end="14831600000t" region="Region_01" xml:id="subtitle369"><span style="style_2"># y el destino trajo muerte.</span></p>
    <p begin="14839200000t" end="14878000000t" region="Region_01" xml:id="subtitle370"><span style="style_2"># María, mi vida, mi amor.</span></p>
    <p begin="14880800000t" end="14924400000t" region="Region_01" xml:id="subtitle371"><span style="style_2"># No dejaré de quererte,</span></p>
    <p begin="14925200000t" end="14962800000t" region="Region_01" xml:id="subtitle372"><span style="style_2"># a balazos te perdí,</span></p>
    <p begin="14967600000t" end="15008400000t" region="Region_01" xml:id="subtitle373"><span style="style_2"># ya no volverás a verme. #</span></p>
    <p begin="15011600000t" end="15037600000t" region="Region_01" xml:id="subtitle374"><span style="style_2">(RÍEN Y VITOREAN)</span></p>
    <p begin="15212000000t" end="15220800000t" region="Region_01" xml:id="subtitle375"><span style="style_2">(BERLÍN) En pie.</span></p>
    <p begin="15234800000t" end="15253600000t" region="Region_01" xml:id="subtitle376"><span style="style_2">Vamos, vamos.</span></p>
    <p begin="15292400000t" end="15309200000t" region="Region_01" xml:id="subtitle377"><span style="style_2">Quítense los antifaces.</span></p>
    <p begin="15327600000t" end="15344800000t" region="Region_01" xml:id="subtitle378"><span style="style_2">¡Quítense los antifaces!</span></p>
    <p begin="15345600000t" end="15356800000t" region="Region_01" xml:id="subtitle379"><span style="style_2">-¿Qué pasa?</span></p>
    <p begin="15363200000t" end="15376800000t" region="Region_01" xml:id="subtitle380"><span style="style_2">-Vamos a ser buenos.</span></p>
    <p begin="15406400000t" end="15427200000t" region="Region_01" xml:id="subtitle381"><span style="style_2">Hemos tenido algún imprevisto,</span></p>
    <p begin="15428000000t" end="15452400000t" region="Region_01" xml:id="subtitle382"><span style="style_2">pero, a pesar de los helicópteros,</span></p>
    <p begin="15453200000t" end="15480800000t" region="Region_00" xml:id="subtitle383"><span style="style_2">me consta que nos van a dar</span><br/><span style="style_2">algunas horas de tregua</span></p>
    <p begin="15482000000t" end="15498800000t" region="Region_01" xml:id="subtitle384"><span style="style_2">y podrán descansar.</span></p>
    <p begin="15502800000t" end="15533600000t" region="Region_00" xml:id="subtitle385"><span style="style_2">En unos minutos</span><br/><span style="style_2">os vamos a repartir sacos de dormir,</span></p>
    <p begin="15538000000t" end="15577200000t" region="Region_00" xml:id="subtitle386"><span style="style_2">agua y un sándwich.</span><br/><span style="style_2">Ah, y les voy a pedir un favor:</span></p>
    <p begin="15590800000t" end="15606000000t" region="Region_01" xml:id="subtitle387"><span style="style_2">quiero que se desnuden.</span></p>
    <p begin="15611600000t" end="15627600000t" region="Region_01" xml:id="subtitle388"><span style="style_2">(MURMURAN)</span></p>
    <p begin="15628400000t" end="15655200000t" region="Region_00" xml:id="subtitle389"><span style="style_2">-Quiero repartirles</span><br/><span style="style_2">un mono rojo como el nuestro</span></p>
    <p begin="15656000000t" end="15673200000t" region="Region_01" xml:id="subtitle390"><span style="style_2">para que se sientan más cómodos.</span></p>
    <p begin="15678800000t" end="15704400000t" region="Region_00" xml:id="subtitle391"><span style="style_5">Perdón, señor,</span><br/><span style="style_5">sin ánimo de molestar.</span></p>
    <p begin="15705200000t" end="15748800000t" region="Region_00" xml:id="subtitle392"><span style="style_5">Entre esta gente hay</span><br/><span style="style_5">enfermos con dolencias cardíacas,</span></p>
    <p begin="15749600000t" end="15782400000t" region="Region_00" xml:id="subtitle393"><span style="style_5">mujeres embarazadas,</span><br/><span style="style_5">diabéticos, adolescentes...</span></p>
    <p begin="15798000000t" end="15810000000t" region="Region_01" xml:id="subtitle394"><span style="style_5">Yo les ruego que, por favor,</span></p>
    <p begin="15810800000t" end="15828800000t" region="Region_00" xml:id="subtitle395"><span style="style_5">dejen marchar</span><br/><span style="style_5">a los más vulnerables.</span></p>
    <p begin="15829600000t" end="15855200000t" region="Region_00" xml:id="subtitle396"><span style="style_5">No creo que pudieran aguantar</span><br/><span style="style_5">esta angustia toda la noche.</span></p>
    <p begin="15856000000t" end="15891200000t" region="Region_00" xml:id="subtitle397"><span style="style_2">-¿Quién te piensas que eres, Gandhi?</span><br/><span style="style_2">(BERLÍN) Denver, tranquilo.</span></p>
    <p begin="15892000000t" end="15930800000t" region="Region_00" xml:id="subtitle398"><span style="style_2">Es un amigo mío. Compartimos</span><br/><span style="style_2">una gran afición por el cine.</span></p>
    <p begin="15986000000t" end="16011200000t" region="Region_02" xml:id="subtitle399"><span style="style_2">(Gritos y sollozos)</span></p>
    <p begin="16032800000t" end="16044800000t" region="Region_01" xml:id="subtitle400"><span style="style_2">-Coge la pistola.</span></p>
    <p begin="16060000000t" end="16077600000t" region="Region_01" xml:id="subtitle401"><span style="style_2">No te pregunto que si quieres,</span></p>
    <p begin="16078400000t" end="16112800000t" region="Region_00" xml:id="subtitle402"><span style="style_2">te digo que cojas la pistola.</span><br/><span style="style_2">Coge la pistola. Coge la pistola.</span></p>
    <p begin="16146000000t" end="16164400000t" region="Region_00" xml:id="subtitle403"><span style="style_2">Y ahora me apuntas.</span><br/><span style="style_5">No, por favor.</span></p>
    <p begin="16165200000t" end="16182800000t" region="Region_01" xml:id="subtitle404"><span style="style_2">Te estoy diciendo que me apuntes.</span></p>
    <p begin="16195600000t" end="16212000000t" region="Region_01" xml:id="subtitle405"><span style="style_2">Una puta orden. Apúntame, coño.</span></p>
    <p begin="16212800000t" end="16228000000t" region="Region_00" xml:id="subtitle406"><span style="style_2">Aquí. Bien.</span><br/><span style="style_5">No, por favor.</span></p>
    <p begin="16233600000t" end="16249200000t" region="Region_01" xml:id="subtitle407"><span style="style_2">Y ahora me disparas.</span></p>
    <p begin="16250800000t" end="16267200000t" region="Region_00" xml:id="subtitle408"><span style="style_5">¿Cómo?</span><br/><span style="style_2">Que me dispares.</span></p>
    <p begin="16276800000t" end="16289600000t" region="Region_00" xml:id="subtitle409"><span style="style_2">Dispárame.</span><br/><span style="style_5">Por favor.</span></p>
    <p begin="16293600000t" end="16316000000t" region="Region_00" xml:id="subtitle410"><span style="style_5">¡Por favor, no!</span><br/><span style="style_2">O me disparas tú</span></p>
    <p begin="16316800000t" end="16336000000t" region="Region_00" xml:id="subtitle411"><span style="style_2">o te disparo yo.</span><br/><span style="style_2">Te regalo diez segundos.</span></p>
    <p begin="16336800000t" end="16368800000t" region="Region_01" xml:id="subtitle412"><span style="style_2">Uno, dos, tres, cuatro, cinco,</span></p>
    <p begin="16369600000t" end="16392400000t" region="Region_01" xml:id="subtitle413"><span style="style_2">seis, siete, ocho, nueve...</span></p>
    <p begin="16393200000t" end="16406800000t" region="Region_01" xml:id="subtitle414"><span style="style_2">(GRITAN)</span></p>
    <p begin="16407600000t" end="16422000000t" region="Region_02" xml:id="subtitle415"><span style="style_2">(Percutor)</span></p>
    <p begin="16454400000t" end="16481200000t" region="Region_00" xml:id="subtitle416"><span style="style_2">-Son falsas, Arturito,</span><br/><span style="style_2">pero lo has hecho muy bien.</span></p>
    <p begin="16482000000t" end="16512400000t" region="Region_00" xml:id="subtitle417"><span style="style_2">Lo has hecho muy bien.</span><br/><span style="style_2">Te la regalo, te la puedes quedar.</span></p>
    <p begin="16513200000t" end="16542000000t" region="Region_00" xml:id="subtitle418"><span style="style_2">-Ahora les vamos a repartir</span><br/><span style="style_2">algunas armas falsas.</span></p>
    <p begin="16542800000t" end="16567200000t" region="Region_00" xml:id="subtitle419"><span style="style_2">En unas horas</span><br/><span style="style_2">precisaremos de su colaboración.</span></p>
    <p begin="16568000000t" end="16604000000t" region="Region_00" xml:id="subtitle420"><span style="style_2">Como han visto, lo único</span><br/><span style="style_2">que tienen que hacer es obedecer.</span></p>
    <p begin="16620000000t" end="16658400000t" region="Region_01" xml:id="subtitle421"><span style="style_2">Confiar en nosotros... y obedecer.</span></p>
    <p begin="16676800000t" end="16694400000t" region="Region_01" xml:id="subtitle422"><span style="style_2">Venga, desnúdense.</span></p>
    <p begin="16706400000t" end="16716800000t" region="Region_01" xml:id="subtitle423"><span style="style_2">Vamos.</span></p>
    <p begin="16817200000t" end="16829200000t" region="Region_01" xml:id="subtitle424"><span style="style_1">"¿Qué lleva puesto?".</span></p>
    <p begin="16830000000t" end="16863600000t" region="Region_00" xml:id="subtitle425"><span style="style_2">Raquel, ni "walkies", ni móviles,</span><br/><span style="style_2">ni aparatos electrónicos.</span></p>
    <p begin="16864400000t" end="16888800000t" region="Region_00" xml:id="subtitle426"><span style="style_2">No hemos rastreado dentro</span><br/><span style="style_2">ninguna radiofrecuencia.</span></p>
    <p begin="16893200000t" end="16924800000t" region="Region_00" xml:id="subtitle427"><span style="style_2">-¿Me estás diciendo que se comunican</span><br/><span style="style_2">con un vaso de yogur y un hilo</span></p>
    <p begin="16925600000t" end="16946000000t" region="Region_00" xml:id="subtitle428"><span style="style_2">para que no los detectemos?</span><br/><span style="style_2">-No sé si es de yogur</span></p>
    <p begin="16946800000t" end="16975600000t" region="Region_00" xml:id="subtitle429"><span style="style_2">o son una banda de sordomudos,</span><br/><span style="style_2">pero no encontramos frecuencias.</span></p>
    <p begin="17208000000t" end="17230400000t" region="Region_02" xml:id="subtitle430"><span style="style_2">(Helicópteros)</span></p>
    <p begin="17524800000t" end="17546800000t" region="Region_00" xml:id="subtitle431"><span style="style_2">¿Qué cojones hacen aquí</span><br/><span style="style_2">esos blindados?</span></p>
    <p begin="17582400000t" end="17602800000t" region="Region_01" xml:id="subtitle432"><span style="style_2">-Vamos a entrar con todo.</span></p>
    <p begin="17611200000t" end="17632800000t" region="Region_01" xml:id="subtitle433"><span style="style_2">-Eso será si yo doy la orden.</span></p>
    <p begin="17633600000t" end="17656000000t" region="Region_00" xml:id="subtitle434"><span style="style_2">Estoy al mando</span><br/><span style="style_2">y todavía no lo he hecho.</span></p>
    <p begin="17684400000t" end="17699600000t" region="Region_01" xml:id="subtitle435"><span style="style_2">-Son órdenes de arriba.</span></p>
    <p begin="17720400000t" end="17742800000t" region="Region_00" xml:id="subtitle436"><span style="style_2">-¿Alguien me va a decir</span><br/><span style="style_2">qué coño está pasando aquí</span></p>
    <p begin="17743600000t" end="17771600000t" region="Region_00" xml:id="subtitle437"><span style="style_2">o tengo que llamar al ministro</span><br/><span style="style_2">de Interior personalmente?</span></p>
    <p begin="17800400000t" end="17822000000t" region="Region_00" xml:id="subtitle438"><span style="style_2">Que qué cojones está pasando.</span><br/><span style="style_2">-Vamos.</span></p>
    <p begin="17918800000t" end="17937600000t" region="Region_00" xml:id="subtitle439"><span style="style_2">Ahí dentro hay</span><br/><span style="style_2">una persona de prioridad 1.</span></p>
    <p begin="17938400000t" end="17962400000t" region="Region_00" xml:id="subtitle440"><span style="style_2">¿Sabe usted lo que es "prioridad 1"?</span><br/><span style="style_2">-No.</span></p>
    <p begin="17963200000t" end="17998000000t" region="Region_00" xml:id="subtitle441"><span style="style_2">No soy de su pandilla de</span><br/><span style="style_2">Inteligencia, no sé qué significa.</span></p>
    <p begin="18001200000t" end="18036400000t" region="Region_00" xml:id="subtitle442"><span style="style_1">Entrarán. Entrarán</span><br/><span style="style_1">porque tenemos a nuestro corderito.</span></p>
    <p begin="18058800000t" end="18073200000t" region="Region_01" xml:id="subtitle443"><span style="style_1">Alison Parker.</span></p>
    <p begin="18074000000t" end="18106000000t" region="Region_00" xml:id="subtitle444"><span style="style_2">(PRIETO) "Hija de 'sir' Benjamin</span><br/><span style="style_2">Parker, embajador del Reino Unido"</span></p>
    <p begin="18106800000t" end="18128400000t" region="Region_00" xml:id="subtitle445"><span style="style_2">y amigo íntimo</span><br/><span style="style_2">de la reina de Inglaterra.</span></p>
    <p begin="18129200000t" end="18150400000t" region="Region_00" xml:id="subtitle446"><span style="style_2">Por eso no tenemos más remedio</span><br/><span style="style_2">que entrar,</span></p>
    <p begin="18151200000t" end="18183600000t" region="Region_00" xml:id="subtitle447"><span style="style_2">porque, si no lo hacemos ahora,</span><br/><span style="style_2">mañana lo sabrá todo el país.</span></p>
    <p begin="18193200000t" end="18233200000t" region="Region_00" xml:id="subtitle448"><span style="style_1">Creerán que no lo sabemos.</span><br/><span style="style_1">Creerán que han conseguido ocultar</span></p>
    <p begin="18234000000t" end="18260400000t" region="Region_00" xml:id="subtitle449"><span style="style_1">toda esta información</span><br/><span style="style_1">a la opinión pública.</span></p>
    <p begin="18261200000t" end="18298800000t" region="Region_00" xml:id="subtitle450"><span style="style_2">(PRIETO) Y hoy es un rehén más,</span><br/><span style="style_2">pero mañana será su principal diana</span></p>
    <p begin="18299600000t" end="18324000000t" region="Region_00" xml:id="subtitle451"><span style="style_2">y todas las pistolas</span><br/><span style="style_2">apuntarán a su cabeza.</span></p>
    <p begin="18327600000t" end="18346000000t" region="Region_01" xml:id="subtitle452"><span style="style_2">¿Es eso lo que quiere? ¿Eh?</span></p>
    <p begin="18357200000t" end="18380800000t" region="Region_01" xml:id="subtitle453"><span style="style_1">Por eso entrarán la primera noche.</span></p>
    <p begin="18381600000t" end="18404000000t" region="Region_01" xml:id="subtitle454"><span style="style_1">Y lo harán antes de las 04:15.</span></p>
    <p begin="18411600000t" end="18441600000t" region="Region_01" xml:id="subtitle455"><span style="style_1">Porque a las 06:30 se hace de día.</span></p>
    <p begin="18442400000t" end="18472400000t" region="Region_00" xml:id="subtitle456"><span style="style_2">¿Se imagina usted lo que sería</span><br/><span style="style_2">para España una crisis con rehenes</span></p>
    <p begin="18473200000t" end="18490800000t" region="Region_01" xml:id="subtitle457"><span style="style_2">gestionada por el Reino Unido?</span></p>
    <p begin="18498400000t" end="18522400000t" region="Region_00" xml:id="subtitle458"><span style="style_1">Y más nos vale</span><br/><span style="style_1">que entren sin pensárselo mucho</span></p>
    <p begin="18523200000t" end="18546400000t" region="Region_00" xml:id="subtitle459"><span style="style_1">porque así tendremos</span><br/><span style="style_1">más posibilidades...</span></p>
    <p begin="18551600000t" end="18567600000t" region="Region_01" xml:id="subtitle460"><span style="style_1">de ganar la primera batalla.</span></p>
    <p begin="18647600000t" end="18658400000t" region="Region_01" xml:id="subtitle461"><span style="style_2">Inspectora Murillo,</span></p>
    <p begin="18659200000t" end="18694400000t" region="Region_00" xml:id="subtitle462"><span style="style_2">vamos a entrar ahora</span><br/><span style="style_2">y vamos a sacar viva a esa chica.</span></p>
    <p begin="18904400000t" end="18939600000t" region="Region_00" xml:id="subtitle463"><span style="style_5">Tengo que hablar contigo...</span><br/><span style="style_5">Llevas ignorándome todo el rato.</span></p>
    <p begin="18941600000t" end="18962800000t" region="Region_00" xml:id="subtitle464"><span style="style_6">Cállate, que nos van a matar</span><br/><span style="style_6">a todos por tu culpa.</span></p>
    <p begin="18963600000t" end="18990000000t" region="Region_00" xml:id="subtitle465"><span style="style_2">(ALGUIEN CHISTA)</span><br/><span style="style_5">Perdón.</span></p>
    <p begin="19156800000t" end="19176800000t" region="Region_00" xml:id="subtitle466"><span style="style_2">Parece que tu jefe</span><br/><span style="style_2">no te cae muy bien, ¿no?</span></p>
    <p begin="19199600000t" end="19213600000t" region="Region_01" xml:id="subtitle467"><span style="style_6">¿Cómo sabes que es mi jefe?</span></p>
    <p begin="19246800000t" end="19266800000t" region="Region_01" xml:id="subtitle468"><span style="style_2">Porque lo sé todo de vosotros.</span></p>
    <p begin="19274400000t" end="19285600000t" region="Region_01" xml:id="subtitle469"><span style="style_2">Lo he estudiado.</span></p>
    <p begin="19304400000t" end="19329600000t" region="Region_00" xml:id="subtitle470"><span style="style_2">También he visto el Predictor</span><br/><span style="style_2">que está en tu mesa.</span></p>
    <p begin="19353600000t" end="19378800000t" region="Region_00" xml:id="subtitle471"><span style="style_2">Y ahora me imagino</span><br/><span style="style_2">quién puede ser el padre.</span></p>
    <p begin="19411600000t" end="19426000000t" region="Region_01" xml:id="subtitle472"><span style="style_2">No le ha gustado el regalito.</span></p>
    <p begin="19458400000t" end="19469200000t" region="Region_01" xml:id="subtitle473"><span style="style_2">¿Qué te ha dicho?</span></p>
    <p begin="19474400000t" end="19489600000t" region="Region_01" xml:id="subtitle474"><span style="style_2">¿Que no se quiere hacer cargo?</span></p>
    <p begin="19560000000t" end="19580400000t" region="Region_01" xml:id="subtitle475"><span style="style_2">No es fácil decirle adiós a un bebé.</span></p>
    <p begin="19667200000t" end="19682000000t" region="Region_01" xml:id="subtitle476"><span style="style_2">¿Qué piensas hacer?</span></p>
    <p begin="19736400000t" end="19745600000t" region="Region_01" xml:id="subtitle477"><span style="style_6">Abortar.</span></p>
    <p begin="19923200000t" end="19953200000t" region="Region_00" xml:id="subtitle478"><span style="style_2">Unidades de intervención 1, 2 y 3,</span><br/><span style="style_2">preparadas.</span></p>
    <p begin="19962000000t" end="19980400000t" region="Region_00" xml:id="subtitle479"><span style="style_2">-Vamos a entrar.</span><br/><span style="style_2">-¡Vamos!</span></p>
    <p begin="19981200000t" end="20007600000t" region="Region_00" xml:id="subtitle480"><span style="style_2">-¡Vamos, vamos, vamos!</span><br/><span style="style_2">-"Equipos 1, 2 y 3, preparados".</span></p>
    <p begin="20030000000t" end="20038400000t" region="Region_01" xml:id="subtitle481"><span style="style_2">-Ahora.</span></p>
    <p begin="20054000000t" end="20072800000t" region="Region_02" xml:id="subtitle482"><span style="style_2">(Sirenas)</span></p>
    <p begin="20169600000t" end="20197200000t" region="Region_00" xml:id="subtitle483"><span style="style_2">Equipo 1,</span><br/><span style="style_2">parapetados en la puerta principal.</span></p>
    <p begin="20198000000t" end="20217200000t" region="Region_00" xml:id="subtitle484"><span style="style_2">Vigilad la azotea</span><br/><span style="style_2">y los flancos altos.</span></p>
    <p begin="20265200000t" end="20281200000t" region="Region_02" xml:id="subtitle485"><span style="style_2">(Teléfono)</span></p>
    <p begin="20314400000t" end="20324800000t" region="Region_01" xml:id="subtitle486"><span style="style_2">Sí.</span></p>
    <p begin="20325600000t" end="20357600000t" region="Region_01" xml:id="subtitle487"><span style="style_1">Berlín, van a entrar.</span></p>
    <p begin="20358400000t" end="20370400000t" region="Region_01" xml:id="subtitle488"><span style="style_2">Allá vamos.</span></p>
    <p begin="20392000000t" end="20420800000t" region="Region_01" xml:id="subtitle489"><span style="style_2">¡Señores, es la hora!</span></p>
    <p begin="20425600000t" end="20449600000t" region="Region_00" xml:id="subtitle490"><span style="style_2">¡Ha llegado el momento</span><br/><span style="style_2">de seguir mis órdenes!</span></p>
    <p begin="20528800000t" end="20539200000t" region="Region_01" xml:id="subtitle491"><span style="style_2">Con nosotros.</span></p>
    <p begin="20622000000t" end="20647200000t" region="Region_00" xml:id="subtitle492"><span style="style_4">"Y así fue como las Fuerzas</span><br/><span style="style_4">de Seguridad del Estado</span></p>
    <p begin="20648000000t" end="20668000000t" region="Region_00" xml:id="subtitle493"><span style="style_4">hicieron exactamente</span><br/><span style="style_4">lo que el Profesor</span></p>
    <p begin="20668800000t" end="20682400000t" region="Region_01" xml:id="subtitle494"><span style="style_4">nos había dicho que harían,</span></p>
    <p begin="20683200000t" end="20707600000t" region="Region_00" xml:id="subtitle495"><span style="style_4">solo que aquella tarde,</span><br/><span style="style_4">cuando lo dijo,</span></p>
    <p begin="20708400000t" end="20732800000t" region="Region_00" xml:id="subtitle496"><span style="style_4">estábamos en el campo</span><br/><span style="style_4">y no teníamos la sensación</span></p>
    <p begin="20733600000t" end="20751600000t" region="Region_00" xml:id="subtitle497"><span style="style_4">de que nos iban a meter</span><br/><span style="style_4">un tiro en la sien".</span></p>
    <p begin="20752400000t" end="20781600000t" region="Region_00" xml:id="subtitle498"><span style="style_1">Entrarán por los cuatro sitios</span><br/><span style="style_1">por los que se puede acceder:</span></p>
    <p begin="20782400000t" end="20800800000t" region="Region_00" xml:id="subtitle499"><span style="style_1">la puerta principal,</span><br/><span style="style_1">la zona de carga,</span></p>
    <p begin="20801600000t" end="20817200000t" region="Region_00" xml:id="subtitle500"><span style="style_1">la salida de emergencia</span><br/><span style="style_1">y la azotea.</span></p>
    <p begin="20818000000t" end="20842800000t" region="Region_00" xml:id="subtitle501"><span style="style_2">(SUÁREZ) Vigilad la retaguardia</span><br/><span style="style_2">de las puertas rebasadas.</span></p>
    <p begin="20843600000t" end="20854800000t" region="Region_01" xml:id="subtitle502"><span style="style_2">Que no salga nadie.</span></p>
    <p begin="20866800000t" end="20912800000t" region="Region_00" xml:id="subtitle503"><span style="style_1">Pero van a esperar. Van a esperar</span><br/><span style="style_1">a que los de Intervención Técnica</span></p>
    <p begin="20913600000t" end="20934000000t" region="Region_01" xml:id="subtitle504"><span style="style_1">hagan un reconocimiento del terreno.</span></p>
    <p begin="20934800000t" end="20959600000t" region="Region_00" xml:id="subtitle505"><span style="style_1">Eso lo van a hacer</span><br/><span style="style_1">desde el acceso de carga.</span></p>
    <p begin="21132400000t" end="21145600000t" region="Region_01" xml:id="subtitle506"><span style="style_2">Vamos, vamos, vamos.</span></p>
    <p begin="21190400000t" end="21206800000t" region="Region_01" xml:id="subtitle507"><span style="style_1">Vamos, que ya están en la puerta.</span></p>
    <p begin="21327600000t" end="21337200000t" region="Region_01" xml:id="subtitle508"><span style="style_2">-¿OK?</span></p>
    <p begin="21352400000t" end="21367600000t" region="Region_01" xml:id="subtitle509"><span style="style_2">-Introduzcan periscopio.</span></p>
    <p begin="21386400000t" end="21398400000t" region="Region_01" xml:id="subtitle510"><span style="style_2">-Vamos.</span></p>
    <p begin="21485200000t" end="21516400000t" region="Region_00" xml:id="subtitle511"><span style="style_2">-Raquel, está entrando una llamada</span><br/><span style="style_2">desde el móvil de una rehén,</span></p>
    <p begin="21517200000t" end="21528800000t" region="Region_01" xml:id="subtitle512"><span style="style_2">Alison Parker.</span></p>
    <p begin="21529600000t" end="21554000000t" region="Region_00" xml:id="subtitle513"><span style="style_2">(RADIO) "Como saben,</span><br/><span style="style_2">un grupo de secuestradores..."</span></p>
    <p begin="21554800000t" end="21564800000t" region="Region_01" xml:id="subtitle514"><span style="style_2">-Súbelo.</span></p>
    <p begin="21565600000t" end="21600400000t" region="Region_00" xml:id="subtitle515"><span style="style_2">(RADIO) "Tenemos otra oyente</span><br/><span style="style_2">al teléfono para dar su opinión.</span></p>
    <p begin="21601200000t" end="21629200000t" region="Region_00" xml:id="subtitle516"><span style="style_2">-No soy una oyente.</span><br/><span style="style_2">Soy una rehén y estoy secuestrada".</span></p>
    <p begin="21632000000t" end="21662400000t" region="Region_00" xml:id="subtitle517"><span style="style_2">Me llamo Alison Parker. Soy la hija</span><br/><span style="style_2">del embajador del Reino Unido.</span></p>
    <p begin="21663200000t" end="21682400000t" region="Region_00" xml:id="subtitle518"><span style="style_2">-En directo, en Onda Cero.</span><br/><span style="style_2">-"Les llamo"</span></p>
    <p begin="21683200000t" end="21704800000t" region="Region_00" xml:id="subtitle519"><span style="style_2">desde el interior</span><br/><span style="style_2">de la Fábrica de Moneda y Timbre.</span></p>
    <p begin="21705600000t" end="21724800000t" region="Region_00" xml:id="subtitle520"><span style="style_2">-¡Que corten</span><br/><span style="style_2">esa llamada inmediatamente!</span></p>
    <p begin="21725600000t" end="21753600000t" region="Region_00" xml:id="subtitle521"><span style="style_2">¡Ya! ¡Que la corten ya!</span><br/><span style="style_2">-¡Corta la llamada!</span></p>
    <p begin="21754400000t" end="21775200000t" region="Region_00" xml:id="subtitle522"><span style="style_2">-Nos han vestido igual</span><br/><span style="style_2">que los secuestradores.</span></p>
    <p begin="21776000000t" end="21792800000t" region="Region_01" xml:id="subtitle523"><span style="style_2">-Coronel, tenemos visión interior.</span></p>
    <p begin="21829200000t" end="21842800000t" region="Region_01" xml:id="subtitle524"><span style="style_2">-¿Qué cojones es eso?</span></p>
    <p begin="21878400000t" end="21905200000t" region="Region_00" xml:id="subtitle525"><span style="style_2">-Nos han armado</span><br/><span style="style_2">y nos han puesto las mismas caretas.</span></p>
    <p begin="21906000000t" end="21944000000t" region="Region_00" xml:id="subtitle526"><span style="style_2">-Amplíe. Páralo.</span><br/><span style="style_2">-Nadie sabe quién es quién.</span></p>
    <p begin="21944800000t" end="21965200000t" region="Region_00" xml:id="subtitle527"><span style="style_2">-Tiene una ametralladora Browning</span><br/><span style="style_2">de suelo.</span></p>
    <p begin="21966000000t" end="21981200000t" region="Region_01" xml:id="subtitle528"><span style="style_2">-Hijos de puta.</span></p>
    <p begin="21984800000t" end="22004400000t" region="Region_00" xml:id="subtitle529"><span style="style_2">-Si entra la Policía,</span><br/><span style="style_2">mataría a inocentes.</span></p>
    <p begin="22005200000t" end="22030400000t" region="Region_00" xml:id="subtitle530"><span style="style_2">-Atrincherados con los rehenes,</span><br/><span style="style_2">todos vestidos iguales.</span></p>
    <p begin="22031200000t" end="22049200000t" region="Region_00" xml:id="subtitle531"><span style="style_2">Es imposible saber</span><br/><span style="style_2">a quién van a disparar.</span></p>
    <p begin="22062400000t" end="22084400000t" region="Region_00" xml:id="subtitle532"><span style="style_2">-No entren.</span><br/><span style="style_2">(RAQUEL) Pare esto, Suárez.</span></p>
    <p begin="22091200000t" end="22101200000t" region="Region_01" xml:id="subtitle533"><span style="style_2">¡Párelo!</span></p>
    <p begin="22139600000t" end="22160800000t" region="Region_00" xml:id="subtitle534"><span style="style_2">-No entren, por Dios.</span><br/><span style="style_2">(RAQUEL) Párelo.</span></p>
    <p begin="22195600000t" end="22221200000t" region="Region_00" xml:id="subtitle535"><span style="style_2">-Abandonen posiciones.</span><br/><span style="style_2">Abortamos operativo.</span></p>
    <p begin="22402000000t" end="22413600000t" region="Region_01" xml:id="subtitle536"><span style="style_2">-Inspectora...</span></p>
    <p begin="22430800000t" end="22452000000t" region="Region_00" xml:id="subtitle537"><span style="style_2">-Termine usted</span><br/><span style="style_2">lo que ha empezado, coronel.</span></p>
    <p begin="22479600000t" end="22500800000t" region="Region_00" xml:id="subtitle538"><span style="style_2">Yo no quiero tener</span><br/><span style="style_2">nada que ver con esto.</span></p>
    <p begin="22617200000t" end="22645200000t" region="Region_00" xml:id="subtitle539"><span style="style_4">"Y así fue</span><br/><span style="style_4">como ganamos nuestra primera batalla</span></p>
    <p begin="22646000000t" end="22661200000t" region="Region_01" xml:id="subtitle540"><span style="style_4">en nuestra primera noche,</span></p>
    <p begin="22662000000t" end="22688800000t" region="Region_00" xml:id="subtitle541"><span style="style_4">sin entrar en combate</span><br/><span style="style_4">y por sentido común".</span></p>
    <p begin="22689600000t" end="22704000000t" region="Region_02" xml:id="subtitle542"><span style="style_2">( "Novena sinfonía", Beethoven )</span></p>
    <p begin="22704800000t" end="22718800000t" region="Region_01" xml:id="subtitle543"><span style="style_4">"Y 20 minutos después...</span></p>
    <p begin="22726800000t" end="22751200000t" region="Region_00" xml:id="subtitle544"><span style="style_4">empezamos a hacer</span><br/><span style="style_4">lo que habíamos venido a hacer".</span></p>
    <p begin="22760800000t" end="22804000000t" region="Region_00" xml:id="subtitle545"><span style="style_2">Ahora toca lo más bonito,</span><br/><span style="style_2">ahora vamos a trabajar.</span></p>
    <p begin="22809200000t" end="22820400000t" region="Region_01" xml:id="subtitle546"><span style="style_2">Conmigo:</span></p>
    <p begin="22824800000t" end="22867600000t" region="Region_01" xml:id="subtitle547"><span style="style_2">Torres, Sánchez, Biedma,</span></p>
    <p begin="22871200000t" end="22882800000t" region="Region_01" xml:id="subtitle548"><span style="style_2">"Lennon,</span></p>
    <p begin="22887200000t" end="22899600000t" region="Region_01" xml:id="subtitle549"><span style="style_2">Guatsani,</span></p>
    <p begin="22902400000t" end="22927600000t" region="Region_01" xml:id="subtitle550"><span style="style_2">Molina, García,</span></p>
    <p begin="22931600000t" end="22967200000t" region="Region_01" xml:id="subtitle551"><span style="style_2">Domingo, De Gorza, Mateo".</span></p>
    <p begin="22968000000t" end="22991600000t" region="Region_00" xml:id="subtitle552"><span style="style_2">Venga, tira por ahí.</span><br/><span style="style_2">"Rodrigo, Molla.</span></p>
    <p begin="22999200000t" end="23019200000t" region="Region_00" xml:id="subtitle553"><span style="style_2">Pérez".</span><br/><span style="style_2">-Venga.</span></p>
    <p begin="23060800000t" end="23095600000t" region="Region_00" xml:id="subtitle554"><span style="style_2">(NAIROBI) A ver,</span><br/><span style="style_2">quiero las máquinas funcionando</span></p>
    <p begin="23096400000t" end="23130400000t" region="Region_00" xml:id="subtitle555"><span style="style_2">las 24 horas, como si esto fuera</span><br/><span style="style_2">una red de pocholos.</span></p>
    <p begin="23131200000t" end="23156000000t" region="Region_00" xml:id="subtitle556"><span style="style_2">Sabéis, ¿no? "Chiqui pun,</span><br/><span style="style_2">chiqui pun, chiqui pun".</span></p>
    <p begin="23156800000t" end="23197600000t" region="Region_00" xml:id="subtitle557"><span style="style_2">"Cada vez que paramos perdemos medio</span><br/><span style="style_2">millón, así que no vamos a parar".</span></p>
    <p begin="23204000000t" end="23230400000t" region="Region_00" xml:id="subtitle558"><span style="style_2">Vamos a hacer las correcciones</span><br/><span style="style_2">técnicas cada tres horas,</span></p>
    <p begin="23231200000t" end="23247600000t" region="Region_01" xml:id="subtitle559"><span style="style_2">"tanto de tinta como de 'offset'".</span></p>
    <p begin="23254800000t" end="23299600000t" region="Region_00" xml:id="subtitle560"><span style="style_2">Así que ya sabéis:</span><br/><span style="style_2">alegría, fiesta e ilusión.</span></p>
    <p begin="23329600000t" end="23357600000t" region="Region_00" xml:id="subtitle561"><span style="style_4">"El trabajo mejor pagado</span><br/><span style="style_4">de la historia:</span></p>
    <p begin="23358400000t" end="23390800000t" region="Region_00" xml:id="subtitle562"><span style="style_4">2400 millones de euros,</span><br/><span style="style_4">tal vez más.</span></p>
    <p begin="23391600000t" end="23417600000t" region="Region_00" xml:id="subtitle563"><span style="style_4">Todo dependía del tiempo</span><br/><span style="style_4">que pudiéramos aguantar dentro".</span></p>
    <p begin="23418400000t" end="23464000000t" region="Region_00" xml:id="subtitle564"><span style="style_2">(NAIROBI) ¡Madre mía!</span><br/><span style="style_2">¡Qué maravilla, colega! (RÍE)</span></p>
    <p begin="23476400000t" end="23502800000t" region="Region_01" xml:id="subtitle565"><span style="style_2">¡Qué maravilla, "coquinoqui"!</span></p>
    <p begin="23505600000t" end="23525600000t" region="Region_01" xml:id="subtitle566"><span style="style_2">¡Nuestro dinero, tío!</span></p>
    <p begin="23532800000t" end="23544800000t" region="Region_01" xml:id="subtitle567"><span style="style_2">¡Joder!</span></p>
    <p begin="23545600000t" end="23579200000t" region="Region_00" xml:id="subtitle568"><span style="style_2">(CONTINÚA GRITANDO Y RIENDO</span><br/><span style="style_2">A LO LEJOS)</span></p>
    <p begin="23590800000t" end="23627600000t" region="Region_00" xml:id="subtitle569"><span style="style_2">¡Estamos haciendo</span><br/><span style="style_2">nuestra propia pasta, troncos!</span></p>
    <p begin="23628400000t" end="23646400000t" region="Region_02" xml:id="subtitle570"><span style="style_2">(Acaba la música)</span></p>
    <p begin="23755200000t" end="23775200000t" region="Region_01" xml:id="subtitle571"><span style="style_2">¿Me pone un descafeinado, por favor?</span></p>
    <p begin="23838400000t" end="23858800000t" region="Region_00" xml:id="subtitle572"><span style="style_2">-"Hola.</span><br/><span style="style_2">Este es el contestador de Mariví.</span></p>
    <p begin="23859600000t" end="23881200000t" region="Region_00" xml:id="subtitle573"><span style="style_2">Cuando suene el 'pi'</span><br/><span style="style_2">puedes dejar un mensaje".</span></p>
    <p begin="23882000000t" end="23892000000t" region="Region_02" xml:id="subtitle574"><span style="style_2">(Pitido)</span></p>
    <p begin="23892800000t" end="23913200000t" region="Region_00" xml:id="subtitle575"><span style="style_2">-Mamá, acabo de terminar,</span><br/><span style="style_2">son las seis...</span></p>
    <p begin="23914000000t" end="23922800000t" region="Region_05" xml:id="subtitle576"><span style="style_2">(Pitido)</span></p>
    <p begin="23959200000t" end="23987600000t" region="Region_00" xml:id="subtitle577"><span style="style_2">Perdone, ¿no tendrá</span><br/><span style="style_2">un cargador de móvil por ahí?</span></p>
    <p begin="23988400000t" end="24000800000t" region="Region_01" xml:id="subtitle578"><span style="style_2">-No.</span></p>
    <p begin="24052000000t" end="24066000000t" region="Region_01" xml:id="subtitle579"><span style="style_1">¿Quiere usar el mío?</span></p>
    <p begin="24101600000t" end="24118400000t" region="Region_01" xml:id="subtitle580"><span style="style_2">-¿De verdad? ¿No le importa?</span></p>
    <p begin="24119200000t" end="24136000000t" region="Region_00" xml:id="subtitle581"><span style="style_1">No.</span><br/><span style="style_2">¿Seguro?</span></p>
    <p begin="24148800000t" end="24160800000t" region="Region_01" xml:id="subtitle582"><span style="style_2">Gracias.</span></p>
    <p begin="24172000000t" end="24204400000t" region="Region_00" xml:id="subtitle583"><span style="style_2">Tengo que hacer un par de llamadas.</span><br/><span style="style_1">No pasa nada.</span></p>
    <p begin="24205200000t" end="24217200000t" region="Region_01" xml:id="subtitle584"><span style="style_2">Muchas gracias.</span></p>
    </div>
    </body>
    </tt>"#);
    let body_selector = Selector::parse("body").unwrap();
    let selector = Selector::parse("body").unwrap();

    for element in fragment.select(&selector) {
        // println!("{}", 1);
        println!("{:?}", element.value());
        // if element.value().name() == "span" {
        //     println!("{:?}", element.value().attr("value"));
        // }
    }
    // println!("{}", &data[..]);
    println!("{}", "Ran");
    list
}
