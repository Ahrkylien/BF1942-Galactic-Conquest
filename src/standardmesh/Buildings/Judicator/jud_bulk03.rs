subshader "jud_bulk03_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
      materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	materialDiffuse 0.980392 0.592157 0.592157;
	texture "texture/buildings/judicator/jud_floor";
}

subshader "jud_bulk03_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.113726 0.0 0.972549;
	texture "texture/buildings/judicator/jud_bulkhead";
}

subshader "jud_bulk03_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 0.0862745 0.882353;
	texture "texture/buildings/judicator/jud_floornospec";
}

